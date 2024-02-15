(function() {var implementors = {
"embedded_graphics":[["impl&lt;C, BO, const WIDTH: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>, const HEIGHT: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>, const N: <a class=\"primitive\" href=\"https://doc.rust-lang.org/nightly/core/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"embedded_graphics/image/trait.GetPixel.html\" title=\"trait embedded_graphics::image::GetPixel\">GetPixel</a> for <a class=\"struct\" href=\"embedded_graphics/framebuffer/struct.Framebuffer.html\" title=\"struct embedded_graphics::framebuffer::Framebuffer\">Framebuffer</a>&lt;C, C::<a class=\"associatedtype\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html#associatedtype.Raw\" title=\"type embedded_graphics::pixelcolor::PixelColor::Raw\">Raw</a>, BO, WIDTH, HEIGHT, N&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;C::<a class=\"associatedtype\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html#associatedtype.Raw\" title=\"type embedded_graphics::pixelcolor::PixelColor::Raw\">Raw</a>&gt;,\n    BO: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/raw/trait.ByteOrder.html\" title=\"trait embedded_graphics::pixelcolor::raw::ByteOrder\">ByteOrder</a>,\n    for&lt;'a&gt; <a class=\"struct\" href=\"embedded_graphics/iterator/raw/struct.RawDataSlice.html\" title=\"struct embedded_graphics::iterator::raw::RawDataSlice\">RawDataSlice</a>&lt;'a, C::<a class=\"associatedtype\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html#associatedtype.Raw\" title=\"type embedded_graphics::pixelcolor::PixelColor::Raw\">Raw</a>, BO&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>&lt;Item = C::<a class=\"associatedtype\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html#associatedtype.Raw\" title=\"type embedded_graphics::pixelcolor::PixelColor::Raw\">Raw</a>&gt;,</div>"],["impl&lt;'a, C, BO&gt; <a class=\"trait\" href=\"embedded_graphics/image/trait.GetPixel.html\" title=\"trait embedded_graphics::image::GetPixel\">GetPixel</a> for <a class=\"struct\" href=\"embedded_graphics/image/struct.ImageRaw.html\" title=\"struct embedded_graphics::image::ImageRaw\">ImageRaw</a>&lt;'a, C, BO&gt;<div class=\"where\">where\n    C: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/convert/trait.From.html\" title=\"trait core::convert::From\">From</a>&lt;&lt;C as <a class=\"trait\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html\" title=\"trait embedded_graphics::pixelcolor::PixelColor\">PixelColor</a>&gt;::<a class=\"associatedtype\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html#associatedtype.Raw\" title=\"type embedded_graphics::pixelcolor::PixelColor::Raw\">Raw</a>&gt;,\n    BO: <a class=\"trait\" href=\"embedded_graphics/pixelcolor/raw/trait.ByteOrder.html\" title=\"trait embedded_graphics::pixelcolor::raw::ByteOrder\">ByteOrder</a>,\n    <a class=\"struct\" href=\"embedded_graphics/iterator/raw/struct.RawDataSlice.html\" title=\"struct embedded_graphics::iterator::raw::RawDataSlice\">RawDataSlice</a>&lt;'a, C::<a class=\"associatedtype\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html#associatedtype.Raw\" title=\"type embedded_graphics::pixelcolor::PixelColor::Raw\">Raw</a>, BO&gt;: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>&lt;Item = C::<a class=\"associatedtype\" href=\"embedded_graphics/pixelcolor/trait.PixelColor.html#associatedtype.Raw\" title=\"type embedded_graphics::pixelcolor::PixelColor::Raw\">Raw</a>&gt;,</div>"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()