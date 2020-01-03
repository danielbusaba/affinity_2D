// Scales every pixel value in the image to fit the scale 0-255
pub fn saturate(image: &mut image::GrayImage)
{
    // Finds the minimum and maximum pixel values in the image
    let mut min = 255;
    let mut max = 0;
    image.iter().for_each(
        | pixel |
        {
            let pixel = *pixel;
            if pixel > max
            {
                max = pixel;
            }
            if pixel < min
            {
                min = pixel;
            }
        }
    );

    // Sets the scale factor to 255 divided by the current pixel value range
    let mut scale: f64 = 255.0;
    if max != min
    {
        scale /= (max - min) as f64;
    }

    // Shifts each pixel back by the minimum pixel value and then applies the scaling factor
    image.iter_mut().for_each(
        | pixel |
        {
            *pixel = ((*pixel - min) as f64 * scale) as u8;
        }
    );
}
