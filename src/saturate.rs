pub fn saturate(image: &mut image::GrayImage)
{
    let mut min = 255;
    let mut max = 0;
    for i in 0 .. image.width()
    {
        for j in 0 .. image.height()
        {
            let pixel = image.get_pixel(i, j) [0];
            if pixel > max
            {
                max = pixel;
            }
            if pixel < min
            {
                min = pixel;
            }
        }
    }

    let scale: f64 = 255.0 / (max - min) as f64;
    for i in 0 .. image.width()
    {
        for j in 0 .. image.height()
        {
            let pixel = image.get_pixel(i, j) [0];
            (*image).get_pixel_mut(i, j) [0] = ((pixel - min) as f64 * scale) as u8;
        }
    }
}
