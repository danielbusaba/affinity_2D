use rayon::prelude::*;

pub fn div16(image: &mut image::GrayImage)
{
    image.par_iter_mut().for_each(
        | pixel |
        {
            *pixel /= 16;
        }
    );
}
