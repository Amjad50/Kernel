use core::{hint, mem};

use alloc::vec::Vec;

use crate::{
    cpu::{self, idt::InterruptAllSavedState, interrupts},
    memory_management::virtual_memory_mapper,
    process::{syscalls, FxSave},
    sync::spin::mutex::Mutex,
};

use super::{Process, ProcessContext, ProcessState};

static SCHEDULER: Mutex<Scheduler> = Mutex::new(Scheduler::new());

struct Scheduler {
    interrupt_initialized: bool,
    processes: Vec<Process>,
}

impl Scheduler {
    const fn new() -> Self {
        Self {
            interrupt_initialized: false,
            processes: Vec::new(),
        }
    }

    pub fn push_process(&mut self, process: Process) {
        self.processes.push(process);
    }

    fn init_interrupt(&mut self) {
        if self.interrupt_initialized {
            return;
        }
        self.interrupt_initialized = true;

        interrupts::create_scheduler_interrupt(scheduler_interrupt_handler);
        interrupts::create_syscall_interrupt(syscall_interrupt_handler);
    }
}

pub fn push_process(process: Process) {
    SCHEDULER.lock().push_process(process);
}

pub fn schedule() -> ! {
    SCHEDULER.lock().init_interrupt();

    loop {
        let current_cpu = cpu::cpu();
        if current_cpu.context.is_some() {
            loop {
                hint::spin_loop();
            }
        }

        let mut scheduler = SCHEDULER.lock();
        // no context holding, i.e. free to take a new process
        for process in scheduler.processes.iter_mut() {
            match process.state {
                ProcessState::Scheduled if current_cpu.context.is_none() => {
                    // found a process to run
                    current_cpu.push_cli();
                    process.state = ProcessState::Running;
                    process.switch_to_this_vm();
                    current_cpu.process_id = process.id;
                    current_cpu.context = Some(process.context);

                    current_cpu.pop_cli();
                }
                ProcessState::Exited => {
                    // keep the process for one time, it will be deleted later.
                    // this is if we want to do extra stuff later
                }
                _ => {}
            }
        }
        scheduler
            .processes
            .retain(|p| p.state != ProcessState::Exited);
        drop(scheduler);

        if current_cpu.context.is_some() {
            // call scheduler_interrupt_handler
            // we are using interrupts to switch context since it allows us to save the registers of exit, which is
            // very convenient
            // The `sys_exit` syscall changes the context from user to kernel,
            // and because of how we implemented syscalls, the result will be in `rax`, so we tell
            // the compiler to ignore `rax` as it may be clobbered after this call
            unsafe { core::arch::asm!("int 0xff", out("rax") _) }
        } else {
            // no process to run, just wait for interrupts
            unsafe { cpu::halt() };
        }
    }
}

pub fn with_current_process<F, U>(f: F) -> U
where
    F: FnOnce(&mut Process) -> U,
{
    let current_cpu = cpu::cpu();
    let mut scheduler = SCHEDULER.lock();
    // TODO: find a better way to store processes or store process index/id.
    let process = scheduler
        .processes
        .iter_mut()
        .find(|p| p.id == current_cpu.process_id)
        .expect("current process not found");
    f(process)
}

/// Exit the current process, and move the `all_state` to the scheduler.
/// The caller of this function (i.e. interrupt) will use the `all_state` to go back to the scheduler.
/// This function will remove the context from the CPU, and thus the value in `all_state` will be dropped.
pub fn exit_current_process(exit_code: u64, all_state: &mut InterruptAllSavedState) {
    let current_cpu = cpu::cpu();
    let mut scheduler = SCHEDULER.lock();

    // TODO: find a better way to store processes or store process index/id.
    let process = scheduler
        .processes
        .iter_mut()
        .find(|p| p.id == current_cpu.process_id)
        .expect("current process not found");

    assert!(process.state == ProcessState::Running);
    assert!(current_cpu.context.is_some());

    // exit process and go back to scheduler (through the syscall handler)
    current_cpu.push_cli();
    process.exit(exit_code);
    // TODO: notify listeners for this process
    println!("Process {} exited with code {}", process.id, exit_code);

    // this will have the state of the cpu in the scheduler
    swap_context(current_cpu.context.as_mut().unwrap(), all_state);
    // drop the cpu context, i.e. drop the current process context
    // the virtual memory will be cleared once we drop the process
    current_cpu.context = None;
    virtual_memory_mapper::switch_to_kernel();
    current_cpu.pop_cli();
}

pub fn yield_current_if_any(all_state: &mut InterruptAllSavedState) {
    let current_cpu = cpu::cpu();
    // do not yield if we don't have context or we are in kernel
    if current_cpu.context.is_none() || all_state.frame.cs & 0x3 == 0 {
        return;
    }
    // save context of this process and mark is as scheduled
    with_current_process(|process| {
        assert!(process.state == ProcessState::Running);
        current_cpu.push_cli();
        swap_context(current_cpu.context.as_mut().unwrap(), all_state);
        // clear context from the CPU
        process.context = current_cpu.context.take().unwrap();
        process.state = ProcessState::Scheduled;
    });
    virtual_memory_mapper::switch_to_kernel();
    current_cpu.pop_cli();
}

pub fn swap_context(context: &mut ProcessContext, all_state: &mut InterruptAllSavedState) {
    let mut fxsave = FxSave::default();
    unsafe { core::arch::x86_64::_fxsave64(&mut fxsave as *mut FxSave as _) };
    unsafe { core::arch::x86_64::_fxrstor64(context.fxsave.0.as_ptr() as _) };
    context.fxsave = fxsave;

    mem::swap(&mut all_state.frame.rflags, &mut context.rflags);
    mem::swap(&mut all_state.frame.rip, &mut context.rip);
    all_state.frame.cs = mem::replace(&mut context.cs, all_state.frame.cs as _) as _;
    mem::swap(&mut all_state.frame.rsp, &mut context.rsp);
    all_state.frame.ss = mem::replace(&mut context.ss, all_state.frame.ss as _) as _;

    mem::swap(&mut all_state.rest.ds, &mut context.ds);
    mem::swap(&mut all_state.rest.es, &mut context.es);
    mem::swap(&mut all_state.rest.fs, &mut context.fs);
    mem::swap(&mut all_state.rest.gs, &mut context.gs);
    mem::swap(&mut all_state.rest.dr0, &mut context.dr0);
    mem::swap(&mut all_state.rest.dr1, &mut context.dr1);
    mem::swap(&mut all_state.rest.dr2, &mut context.dr2);
    mem::swap(&mut all_state.rest.dr3, &mut context.dr3);
    mem::swap(&mut all_state.rest.dr6, &mut context.dr6);
    mem::swap(&mut all_state.rest.dr7, &mut context.dr7);
    mem::swap(&mut all_state.rest.rax, &mut context.rax);
    mem::swap(&mut all_state.rest.rbx, &mut context.rbx);
    mem::swap(&mut all_state.rest.rcx, &mut context.rcx);
    mem::swap(&mut all_state.rest.rdx, &mut context.rdx);
    mem::swap(&mut all_state.rest.rsi, &mut context.rsi);
    mem::swap(&mut all_state.rest.rdi, &mut context.rdi);
    mem::swap(&mut all_state.rest.rbp, &mut context.rbp);
    mem::swap(&mut all_state.rest.r8, &mut context.r8);
    mem::swap(&mut all_state.rest.r9, &mut context.r9);
    mem::swap(&mut all_state.rest.r10, &mut context.r10);
    mem::swap(&mut all_state.rest.r11, &mut context.r11);
    mem::swap(&mut all_state.rest.r12, &mut context.r12);
    mem::swap(&mut all_state.rest.r13, &mut context.r13);
    mem::swap(&mut all_state.rest.r14, &mut context.r14);
    mem::swap(&mut all_state.rest.r15, &mut context.r15);
}

extern "cdecl" fn scheduler_interrupt_handler(all_state: &mut InterruptAllSavedState) {
    assert!(all_state.frame.cs & 0x3 == 0, "must be from kernel only");
    let current_cpu = cpu::cpu();
    assert!(current_cpu.context.is_some());

    swap_context(current_cpu.context.as_mut().unwrap(), all_state);
}

extern "cdecl" fn syscall_interrupt_handler(all_state: &mut InterruptAllSavedState) {
    assert!(all_state.frame.cs & 0x3 == 3, "must be from user only");
    let current_cpu = cpu::cpu();
    assert!(current_cpu.context.is_some());

    syscalls::handle_syscall(all_state);
}
