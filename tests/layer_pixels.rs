use goo::LayerContent;

#[test]
fn roundtrip_pixels() {
    let width = 3;
    let height = 2;
    let pixels = vec![0u8, 1, 2, 3, 4, 5];
    let mut layer = LayerContent::default();
    layer.set_pixels(width, height, &pixels);
    assert_eq!(layer.decode_pixels(width, height), pixels);
}

#[cfg(feature = "image")]
#[test]
fn image_helpers() {
    let width = 2;
    let height = 2;
    let pixels = vec![0u8, 255, 128, 64];
    let mut layer = LayerContent::default();
    layer.set_pixels(width, height, &pixels);
    let img = layer.to_image(width, height);
    assert_eq!(img.width(), width);
    assert_eq!(img.height(), height);
    let mut layer2 = LayerContent::default();
    layer2.set_from_image(&img);
    assert_eq!(layer2.decode_pixels(width, height), pixels);
}
