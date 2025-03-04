# Testing

We have testing framework in the kernel that tries to replicate the testing framework in `rust-std`.

We can't just use `#[test]`, so instead we created our own macro for tests `testing::test!`.

Example:

```rust
testing::test! {
    fn test_free_realloc() {
        let page = unsafe { alloc() };
        let addr = page as usize;

        unsafe { free(page) };

        let page2 = unsafe { alloc() };

        assert_eq!(page as usize, addr);

        unsafe { free(page2) };
    }

    #[should_panic]
    fn test_unaligned_free() {
        let page = unsafe { alloc() };

        let addr_inside_page = unsafe { page.add(1) };

        unsafe { free(addr_inside_page) };
    }
}
```

> We have added `macro_rules_attribute` temporarily, so instead of using `testing::test! {}` we can use this:
>
> ```rust
> #[macro_rules_attribute::apply(testing::test)]
> fn test() {
>     // test code
> }
> ```
>
> The plan is to replace it with custom `proc_macro` crate later.

When you create a new feature be sure to add a test for it as much as possible.

