/* 
 * Project   : moseiik
 * Authors   : Faucheux Valentin and Plumejeau Maxime
 * File      : unit testing of l1_x86_sse2 function
 * Comments  : verfiy correct behavior of l1_x86_sse2 function for the following cases :
 *             - 2 identical images
 *             - 2 completely different images
 *             - 2 slightly different images
 */

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use crate::main::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use image::RgbImage;

#[cfg(test)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod tests {
    use super::*;

    /*
     * Test 1: Verify that 'l1_x86_sse' returns a difference of 0 for two identical images
     */
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn unit_test_l1_x86_sse2_identical_images() {

        // Create a 10x10 pixels image
        let mut im1 = RgbImage::new(10, 10);

        // Fill image im1 with one color
        for pixel in im1.pixels_mut() {
            *pixel = image::Rgb([100, 150, 200]);
        }

        // Clone image im1 in image im2
        let im2 = im1.clone();

        // Verify that the difference is 0 for identical images im1 and im2
        let result = unsafe { l1_x86_sse2(&im1, &im2) };
        assert_eq!(result, 0, "Test failed for identical images with l1_x86_sse2.");
    }

    /*
     * Test 2: Verify that 'l1_x86_sse' calculates the correct difference 
     * for two completely different images of the same size
     */
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn unit_test_l1_x86_sse2_different_images() {

        // Create two 10x10 pixels images
        let mut im1 = RgbImage::new(10, 10);
        let mut im2 = RgbImage::new(10, 10);

        // Create two different colors
        let im1_c: [u8; 3] = [100, 150, 200];
        let im2_c: [u8; 3] = [50, 100, 150];

        // Fill images im1 and im2 with different colors
        for pixel in im1.pixels_mut() {
            *pixel = image::Rgb(im1_c);
        }
        for pixel in im2.pixels_mut() {
            *pixel = image::Rgb(im2_c);
        }

        // Compute the expected difference between images im1 and im2
        let expected_diff =
            ((im1_c[0] as i32 - im2_c[0] as i32).abs() * im1.width() as i32 * im1.height() as i32)
                + ((im1_c[1] as i32 - im2_c[1] as i32).abs()
                    * im1.width() as i32
                    * im1.height() as i32)
                + ((im1_c[2] as i32 - im2_c[2] as i32).abs()
                    * im1.width() as i32
                    * im1.height() as i32);

        // Verify that the difference between images im1 and im2 is the expected one
        // Expected difference = sum of absolute differences across all pixels for RGB channels
        let result = unsafe { l1_x86_sse2(&im1, &im2) };
        assert_eq!(result, expected_diff, "Test failed for different images with l1_x86_sse2.");
    }

    /*
     * Test 3: Verify that 'l1_x86_sse' calculates the correct difference 
     * for two images with slight differences
     */
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn unit_test_l1_x86_sse2_partial_difference() {

        // Create two 10x10 pixels images
        let mut im1 = RgbImage::new(10, 10);

        // Fill image im1 with one color
        for pixel in im1.pixels_mut() {
            *pixel = image::Rgb([100, 150, 200]);
        }

        // Clone image im1 in image im2 and modify two pixels of image im2
        let mut im2 = im1.clone();
        im2.put_pixel(0, 0, image::Rgb([255, 255, 255]));
        im2.put_pixel(1, 1, image::Rgb([0, 0, 0]));

        // Compute the expected difference between images im1 and im2
        let mut expected_diff = 0;
        expected_diff += (im2.get_pixel(0, 0)[0] as i32 - im1.get_pixel(0, 0)[0] as i32).abs()
            + (im2.get_pixel(0, 0)[1] as i32 - im1.get_pixel(0, 0)[1] as i32).abs()
            + (im2.get_pixel(0, 0)[2] as i32 - im1.get_pixel(0, 0)[2] as i32).abs();

        expected_diff += (im2.get_pixel(1, 1)[0] as i32 - im1.get_pixel(1, 1)[0] as i32).abs()
            + (im2.get_pixel(1, 1)[1] as i32 - im1.get_pixel(1, 1)[1] as i32).abs()
            + (im2.get_pixel(1, 1)[2] as i32 - im1.get_pixel(1, 1)[2] as i32).abs();

        // Verify that the difference between images im1 and im2 is the expected one
        // Expected difference = sum of absolute differences across all pixels for RGB channels
        let result = unsafe { l1_x86_sse2(&im1, &im2) };
        assert_eq!(result, expected_diff, "Test failed for partially different images with l1_x86_sse2.");
    }
}
