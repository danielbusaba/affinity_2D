pub fn div16(image: &mut image::GrayImage)
{
    for i in 0 .. image.width()
    {
        for j in 0 .. image.height()
        {
            let pixel = image.get_pixel(i, j) [0];
            (*image).get_pixel_mut(i, j) [0] = pixel / 16;
        }
    }
}
