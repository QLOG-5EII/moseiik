/* 
 * Project   : moseiik
 * Authors   : Faucheux Valentin and Plumejeau Maxime
 * File      : unit testing of l1_generic function
 * Comments  : verfiy correct behavior of l1_generic function for the following cases :
 *             - 2 identical images
 *             - 2 completely different images
 *             - 2 slightly different images
 *             - 2 different 1x1 pixel images
 */

use crate::main::*;
use image::RgbImage;

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * Test 1: Verify that `l1_generic` returns a difference of 0 for two identical images
     */
    #[test]
    fn unit_test_l1_generic_identical_images() {

        // Create a 2x2 pixels image
        let mut im1 = RgbImage::new(2, 2);

        // Fill image im1 with one color
        let color = image::Rgb([100, 150, 200]);
        for pixel in im1.pixels_mut() {
            *pixel = color;
        }

        // Clone image im1 in image im2
        let im2 = im1.clone();

        // Verify that the difference is 0 for identical images im1 and im2
        let expected_diff = 0;
        let result = l1_generic(&im1, &im2);
        assert_eq!(result, expected_diff, "Test failed for identical images.");
    }

    /*
     * Test 2: Verify that `l1_generic` calculates the correct difference 
     * for two completely different images of the same size
     */
    #[test]
    fn unit_test_l1_generic_completely_different_images() {

        // Create two 2x2 pixels images
        let mut im1 = RgbImage::new(2, 2);
        let mut im2 = RgbImage::new(2, 2);

        // Fill image im1 with one color
        let color1 = image::Rgb([100, 150, 200]);
        for pixel in im1.pixels_mut() {
            *pixel = color1;
        }

        // Fill image im2 with a different color
        let color2 = image::Rgb([50, 100, 150]);
        for pixel in im2.pixels_mut() {
            *pixel = color2;
        }

        // Verify that the difference between images im1 and im2 is 600
        // Expected difference = sum of absolute differences across all pixels for RGB channels
        let expected_diff = 600;
        let result = l1_generic(&im1, &im2);
        assert_eq!(result, expected_diff, "Test failed for completely different images.");
    }

    /*
     * Test 3: Verify that `l1_generic` calculates the correct difference 
     * for two images differing by only a single pixel
     */
    #[test]
    fn unit_test_l1_generic_slightly_different_images() {

        // Create a 2x2 pixel image
        let mut im1 = RgbImage::new(2, 2);

        // Fill image im1 with one color
        let color1 = image::Rgb([100, 150, 200]);
        for pixel in im1.pixels_mut() {
            *pixel = color1;
        }

        // Clone image im1 in image im2 and slightly modify a single pixel of image im2
        let color2 = image::Rgb([110, 150, 200]);
        let mut im2 = im1.clone();
        im2.put_pixel(0, 0, color2);

        // Verify that the difference between images im1 and im2 is 10 
        // Expected difference = sum of absolute differences across all pixels for RGB channels
        let expected_diff = 10;
        let result = l1_generic(&im1, &im2);
        assert_eq!(result, expected_diff, "Test failed for slightly different images."
        );
    }

    /*
     * Test 4: Verify that `l1_generic` calculates the correct difference for images of size 1x1 pixel
     */
    #[test]
    fn unit_test_l1_generic_single_pixel() {

        // Create two 1x1 pixel images
        let mut im1 = RgbImage::new(1, 1);
        let mut im2 = RgbImage::new(1, 1);

        // Set a color for image im1
        let color1 = image::Rgb([100, 150, 200]);
        im1.put_pixel(0, 0, color1);

        // Set a different color for image im2
        let color2 = image::Rgb([50, 100, 150]);
        im2.put_pixel(0, 0, color2);

        // Verify that the difference between images im1 and im2 is 150
        // Expected difference = sum of absolute differences for RGB channels
        let expected_diff = 150;
        let result = l1_generic(&im1, &im2);
        assert_eq!(result, expected_diff, "Test failed for single pixel images."
        );
    }
}
