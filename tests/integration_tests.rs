/* 
 * Project   : moseiik
 * Authors   : Faucheux Valentin and Plumejeau Maxime
 * File      : integration_tests of compute_moseiik function
 * Comments  : verfiy correct behavior of compute_moseiik function for the input image 'kit.jpeg' and ouptut image 'output.png'.
 */

use moseiik::main::compute_mosaic;
use moseiik::main::Options;

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * Test 1: Verify that 'compute_moseiik' generates the correct image
     * for the input image 'kit.jpeg' and ouptut image 'output.png' on x86 target.
     */
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        // Set options
        let args = Options {
            image: "assets/kit.jpeg".to_string(),
            tiles: "assets/images/".to_string(),
            output: "output_x86.png".to_string(),
            tile_size: 5,
            scaling: 1,
            remove_used: false,
            verbose: false,
            simd: true, // SIMD specific to x86
            num_thread: 1,
        };

        compute_mosaic(args); //Compute the moseiik image

        let generated = image::open("output_x86.png").expect("Failed to open generated image");
        let ground_truth = image::open("assets/output.png").expect("Failed to open expected image");

        // Compare the generated image with the expected image
        assert_eq!(
            generated, ground_truth,
            "Generated image does not match the expect one!"
        );
    }

    /*
     * Test 2: Verify that 'compute_moseiik' generates the correct image
     * for the input image 'kit.jpeg' and ouptut image 'output.png' on aarch64 target.
     */
    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        // Set options
        let args = Options {
            image: "assets/kit.jpeg".to_string(),
            tiles: "assets/images/".to_string(),
            output: "output_aarch64.png".to_string(),
            tile_size: 5,
            scaling: 1,
            remove_used: false,
            verbose: false,
            simd: true, // SIMD specific to aarch64
            num_thread: 1,
        };

        compute_mosaic(args); //Compute the moseiik image

        let generated = image::open("output_aarch64.png").expect("Failed to open generated image");
        let ground_truth = image::open("assets/output.png").expect("Failed to open the expected image");

        // Compare the generated image with the expected image
        assert_eq!(
            generated, ground_truth,
            "Generated image does not match!"
        );
    }

    /*
     * Test 3: Verify that 'compute_moseiik' generates the correct image
     * for the input image 'kit.jpeg' and ouptut image 'output.png'.
     */
    #[test]
    fn test_generic() {
        // Set options
        let args = Options {
            image: "assets/kit.jpeg".to_string(),
            tiles: "assets/images/".to_string(),
            output: "output_generic.png".to_string(),
            tile_size: 5,
            scaling: 1,
            remove_used: false,
            verbose: false,
            simd: false, // Generic version
            num_thread: 1,
        };

        compute_mosaic(args); //Compute the moseiik image

        let generated = image::open("output_generic.png").expect("Failed to open generated image");
        let ground_truth = image::open("assets/output.png").expect("Failed to open the expected image");

        // Compare the generated image with the expected image
        assert_eq!(
            generated, ground_truth,
            "Generated image does not match!"
        );
    }
}
