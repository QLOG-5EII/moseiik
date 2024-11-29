

#[cfg(test)] // This attribute ensures that the module below is only compiled when running tests.
mod tests {
    use std::fs;                        // Import the fs module for file system operations (reading and writing files).
    use std::path::Path;                // Import the Path module to work with file paths.
    use moseiik::main::compute_mosaic;  // Import the compute_mosaic function from the moseiik crate for mosaic generation.
    use moseiik::main::Options;         // Import the Options struct, which holds the configuration for compute_mosaic.

    // Unit test for x86 architecture, ensuring that the test runs only on x86 or x86_64 architectures.
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]    // Ensures the test is run only on x86 or x86_64 architectures.
    fn test_x86() {
        // Paths for the source image, reference image (ground truth), and the generated mosaic.
        let input_image_path = "assets/kit.jpeg";       // Path to the input image.
        let reference_image_path = "assets/ground-truth-kit.png";   // Path to the reference image.
        let output_image_path = "assets/generated-mosaic_x86.png";  // Path where the generated mosaic will be saved.

        // Path for the directory containing tiles to be used for the mosaic.
        let my_tiles = "vignettes/images";
        let my_tile_size = 25; // Size of the tiles to be used (5x5 pixels).

        // Creating the Options struct, which contains all the configuration needed to generate the mosaic.
        let args = Options {
            image: input_image_path.parse().unwrap(),       // Parse the input image path into a valid PathBuf.
            output: (&output_image_path).parse().unwrap(),  // Parse the output image path.
            tiles : my_tiles.parse().unwrap(),              // Parse the tiles directory path.
            scaling : 1,                                    // Scaling factor for the mosaic generation (no scaling in this case).
            tile_size : my_tile_size,                       // The size of each tile.
            remove_used: true,                              // Whether to remove the used tiles after processing.
            verbose : false,                                // Disable verbose output.
            simd : true,                                   // Disable SIMD (Single Instruction Multiple Data) optimizations.
            num_thread : 8,                                 // Use only one thread for processing.
        };

        // Check if the input image and reference image exist before proceeding with the test.
        assert!(Path::new(input_image_path).exists(), "Source image not found");              // Assert that the source image exists.
        assert!(Path::new(reference_image_path).exists(), "Reference image not found");    // Assert that the reference image exists.


        // Call the function compute_mosaic to generate the mosaic using the specified options.
        compute_mosaic(args);   // Generate the mosaic.

        // Read the binary data of the generated mosaic and the reference image to compare them.
        let generated_data = fs::read(output_image_path).expect("Unable to read the generated image");          // Read the generated mosaic image.
        let reference_data = fs::read(reference_image_path).expect("Unable to read the reference image.");  // Read the reference image.

        // Compare the binary data of the generated image and the reference image to verify correctness.
        assert_eq!(
            generated_data, reference_data,
            "The generated image does not match the ground truth"   // Error message if the images do not match.
        );

        // Clean up by deleting the generated mosaic image after the test.
        fs::remove_file(output_image_path).expect("Unable to remove the generated file");   // Delete the output image file.
    }

    // Unit test for ARM architecture, specifically testing with aarch64.
    #[test]
    #[cfg(target_arch = "aarch64")]     // Ensures the test runs only on ARM 64-bit architectures (aarch64).
    fn test_aarch64() {
        // Paths for the source image, reference image (ground truth), and the generated mosaic.
        let input_image_path = "assets/kit.jpeg";       // Path to the input image.
        let reference_image_path = "assets/ground-truth-kit.png";   // Path to the reference image.
        let output_image_path = "assets/generated-mosaic_arm64.png";  // Path where the generated mosaic will be saved.

        // Path for the directory containing tiles to be used for the mosaic.
        let my_tiles = "vignettes/images";
        let my_tile_size = 25; // Size of the tiles to be used (5x5 pixels).

        // Creating the Options struct, which contains all the configuration needed to generate the mosaic.
        let args = Options {
            image: input_image_path.parse().unwrap(),       // Parse the input image path into a valid PathBuf.
            output: (&output_image_path).parse().unwrap(),  // Parse the output image path.
            tiles : my_tiles.parse().unwrap(),              // Parse the tiles directory path.
            scaling : 1,                                    // Scaling factor for the mosaic generation (no scaling in this case).
            tile_size : my_tile_size,                       // The size of each tile.
            remove_used: true,                              // Whether to remove the used tiles after processing.
            verbose : false,                                // Disable verbose output.
            simd : true,                                   // Disable SIMD (Single Instruction Multiple Data) optimizations.
            num_thread : 8,                                 // Use only one thread for processing.
        };

        // Check if the input image and reference image exist before proceeding with the test.
        assert!(Path::new(input_image_path).exists(), "Source image not found");              // Assert that the source image exists.
        assert!(Path::new(reference_image_path).exists(), "Reference image not found");    // Assert that the reference image exists.


        // Call the function compute_mosaic to generate the mosaic using the specified options.
        compute_mosaic(args);   // Generate the mosaic.

        // Read the binary data of the generated mosaic and the reference image to compare them.
        let generated_data = fs::read(output_image_path).expect("Unable to read the generated image");          // Read the generated mosaic image.
        let reference_data = fs::read(reference_image_path).expect("Unable to read the reference image.");  // Read the reference image.

        // Compare the binary data of the generated image and the reference image to verify correctness.
        assert_eq!(
            generated_data, reference_data,
            "The generated image does not match the ground truth"   // Error message if the images do not match.
        );

        // Clean up by deleting the generated mosaic image after the test.
        fs::remove_file(output_image_path).expect("Unable to remove the generated file");   // Delete the output image file.
    }

    // Generic unit test to test the mosaic creation and validation process.
    #[test]
    fn test_generic() {
        // Paths for the source image, reference image (ground truth), and the generated mosaic.
        let input_image_path = "assets/target-small.png";       // Path to the input image.
        let reference_image_path = input_image_path;   // Path to the reference image.
        let output_image_path = "assets/generated-mosaic_generic.png";  // Path where the generated mosaic will be saved.

        // Path for the directory containing tiles to be used for the mosaic.
        let my_tiles = "assets/tiles-small";
        let my_tile_size = 5; // Size of the tiles to be used (5x5 pixels).

        // Creating the Options struct, which contains all the configuration needed to generate the mosaic.
        let args = Options {
            image: input_image_path.parse().unwrap(),       // Parse the input image path into a valid PathBuf.
            output: (&output_image_path).parse().unwrap(),  // Parse the output image path.
            tiles : my_tiles.parse().unwrap(),              // Parse the tiles directory path.
            scaling : 1,                                    // Scaling factor for the mosaic generation (no scaling in this case).
            tile_size : my_tile_size,                       // The size of each tile.
            remove_used: true,                              // Whether to remove the used tiles after processing.
            verbose : false,                                // Disable verbose output.
            simd : false,                                   // Disable SIMD (Single Instruction Multiple Data) optimizations.
            num_thread : 1,                                 // Use only one thread for processing.
        };

        // Check if the input image and reference image exist before proceeding with the test.
        assert!(Path::new(input_image_path).exists(), "Source image not found");              // Assert that the source image exists.
        assert!(Path::new(reference_image_path).exists(), "Reference image not found");    // Assert that the reference image exists.


        // Call the function compute_mosaic to generate the mosaic using the specified options.
        compute_mosaic(args);   // Generate the mosaic.

        // Read the binary data of the generated mosaic and the reference image to compare them.
        let generated_data = fs::read(output_image_path).expect("Unable to read the generated image");          // Read the generated mosaic image.
        let reference_data = fs::read(reference_image_path).expect("Unable to read the reference image.");  // Read the reference image.

        // Compare the binary data of the generated image and the reference image to verify correctness.
        assert_eq!(
            generated_data, reference_data,
            "The generated image does not match the ground truth"   // Error message if the images do not match.
        );

        // Clean up by deleting the generated mosaic image after the test.
        fs::remove_file(output_image_path).expect("Unable to remove the generated file");   // Delete the output image file.
    }
}
