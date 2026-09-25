#[path = "../../graphics/renderer/framebuffer.rs"]
mod framebuffer_impl;

#[test]
fn framebuffer_initializes_expected_dimensions() {
    let framebuffer = framebuffer_impl::Framebuffer::new(4, 2).unwrap();
    assert_eq!(framebuffer.width, 4);
    assert_eq!(framebuffer.height, 2);
    assert_eq!(framebuffer.pixels().len(), 8);
}

#[test]
fn framebuffer_can_clear_and_write_pixels() {
    let mut framebuffer = framebuffer_impl::Framebuffer::new(2, 2).unwrap();
    framebuffer.clear(0x12345678);
    framebuffer.set_pixel(1, 1, 0xFF00FF00).unwrap();

    assert_eq!(framebuffer.pixels()[0], 0x12345678);
    assert_eq!(framebuffer.pixels()[3], 0xFF00FF00);
}
