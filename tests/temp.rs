#[cfg(test)]
mod tests {
    // Import necessary modules and functions
    use moseiik::main::{compute_mosaic, Options, load_image};

    // Test for x86 architecture
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        // Set options for reference image
        let options = Options {
            image: String::from("./assets/kit.jpeg"),
            output: String::from("./assets/Reference_image.png"),
            tiles: String::from("./assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: true,
            num_thread: 1,
        };
        // Generate the mosaic for the reference image
        compute_mosaic(options);

        // Set options for SIMD and non-SIMD tests
        let options_simd = Options {
            image: String::from("./assets/kit.jpeg"),
            output: String::from("./assets/test_x86.png"),
            tiles: String::from("./assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: true,
            num_thread: 1,
        };

        let options_not_simd = Options {
            image: String::from("./assets/kit.jpeg"),
            output: String::from("./assets/test_x86.png"),
            tiles: String::from("./assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: false,
            num_thread: 1,
        };

        // Test with SIMD
        compute_mosaic(options_simd);
        let image_res = load_image("./assets/test_x86.png");
        let image_ref = load_image("./assets/Reference_image.png");
        assert_eq!(image_res, image_ref);

        // Test without SIMD
        compute_mosaic(options_not_simd);
        let image_res = load_image("./assets/test_x86.png");
        let image_ref = load_image("./assets/Reference_image.png");
        assert_eq!(image_res, image_ref);
    }

    // Test for aarch64 architecture
    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        // Set options for reference image
        let options = Options {
            image: String::from("./assets/kit.jpeg"),
            output: String::from("./assets/Reference_image.png"),
            tiles: String::from("./assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: true,
            num_thread: 1,
        };
        // Generate the mosaic for the reference image
        compute_mosaic(options);

        // Set options for SIMD and non-SIMD tests
        let options_simd = Options {
            image: String::from("./assets/kit.jpeg"),
            output: String::from("./assets/test_arch64.png"),
            tiles: String::from("./assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: true,
            num_thread: 1,
        };

        let options_not_simd = Options {
            image: String::from("./assets/kit.jpeg"),
            output: String::from("./assets/test_arch64.png"),
            tiles: String::from("./assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: false,
            num_thread: 1,
        };

        // Test with SIMD
        compute_mosaic(options_simd);
        let image_res = load_image("./assets/test_arch64.png");
        let image_ref = load_image("./assets/Reference_image.png");
        assert_eq!(image_res, image_ref);

        // Test without SIMD
        compute_mosaic(options_not_simd);
        let image_res = load_image("./assets/test_arch64.png");
        let image_ref = load_image("./assets/Reference_image.png");
        assert_eq!(image_res, image_ref);
    }

    // Generic test
    #[test]
    fn test_generic() {
        // Set options for reference image
        let options = Options {
            image: String::from("./assets/kit.jpeg"),
            output: String::from("./assets/Reference_image.png"),
            tiles: String::from("./assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: true,
            num_thread: 1,
        };
        // Generate the mosaic for the reference image
        compute_mosaic(options);

        // Set options for SIMD and non-SIMD tests
        let options_simd = Options {
            image: String::from("./assets/kit.jpeg"),
            output: String::from("./assets/test_generic.png"),
            tiles: String::from("./assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: true,
            num_thread: 1,
        };

        let options_not_simd = Options {
            image: String::from("./assets/kit.jpeg"),
            output: String::from("./assets/test_generic.png"),
            tiles: String::from("./assets/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: false,
            num_thread: 1,
        };

        // Test with SIMD
        compute_mosaic(options_simd);
        let image_res = load_image("./assets/test_generic.png");
        let image_ref = load_image("./assets/Reference_image.png");
        assert_eq!(image_res, image_ref);

        // Test without SIMD
        compute_mosaic(options_not_simd);
        let image_res = load_image("./assets/test_generic.png");
        let image_ref = load_image("./assets/Reference_image.png");
        assert_eq!(image_res, image_ref);
    }
}