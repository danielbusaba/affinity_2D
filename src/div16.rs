// Divides every pixel value in the image by 16 (truncates result)
pub fn div16(image: &mut image::GrayImage)
{
    image.iter_mut().for_each(
        | pixel |
        {
            *pixel /= 16;
        }
    );
}
