    #[cfg(test)]
    mod tests {
        use moseiik::main::{compute_mosaic, Options};
        use image::io::Reader as ImageReader;
        use image::RgbImage;
        #[test]
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        fn test_x86() {
            //simd = true and architecture x86
            // test avx2 or sse2 if available

            //we chose to have the files in specific folder to ensure reliability
            let opts = Options {
                image: "tests/data/input.jpeg".to_string(),
                output: "tests/data/output_x86.png".to_string(),
                tiles: "tests/tiles".to_string(),
                scaling: 1,
                tile_size: 25,
                remove_used: false,
                verbose: false,
                simd: true, //what we test
                num_thread: 1,
            };

            //copying the value because compute_mosaic consumes the structure
            let output_file = opts.output.clone();
            compute_mosaic(opts);

            println!("reference");
            let reference : RgbImage = ImageReader::open("tests/reference/ground-truth-kit.png".to_string()).unwrap().decode().unwrap().into_rgb8();
            println!("output {:?}", output_file);
            let output : RgbImage = ImageReader::open(&output_file).unwrap().decode().unwrap().into_rgb8();

            //verify step by step, first presence, then format, then content
            assert!(std::path::Path::new(&output_file).exists());
            assert_eq!(reference.dimensions(), output.dimensions());
            assert_eq!(reference, output);
        }

        #[test]
        #[cfg(target_arch = "aarch64")]
        fn test_aarch64() {
            //simd = true and architecture arch

            //we chose to have the files in specific folder to ensure reliability
            let opts = Options {
                image: "tests/data/input.jpeg".to_string(),
                output: "tests/data/output_aarch64.png".to_string(),
                tiles: "tests/tiles".to_string(),
                scaling: 1,
                tile_size: 25,
                remove_used: false,
                verbose: false,
                simd: true, //what we test
                num_thread: 1,
            };

            //copying the value because compute_mosaic consumes the structure
            let output_file = opts.output.clone();
            compute_mosaic(opts);

            let reference : RgbImage = ImageReader::open("tests/reference/ground-truth-kit.png".to_string()).unwrap().decode().unwrap().into_rgb8();
            let output : RgbImage = ImageReader::open(&output_file).unwrap().decode().unwrap().into_rgb8();

            //verify step by step, first presence, then format, then content
            assert!(std::path::Path::new(&output_file).exists());
            assert_eq!(reference.dimensions(), output.dimensions());
            assert_eq!(reference, output);
        }

        #[test]
        fn test_generic() {
            //simd = false
            //we chose to have the files in specific folder to ensure reliability
            let opts = Options {
                image: "tests/data/input.jpeg".to_string(),
                output: "tests/data/output_generic.png".to_string(),
                tiles: "tests/tiles".to_string(),
                scaling: 1,
                tile_size: 25,
                remove_used: false,
                verbose: false,
                simd: false, //what we test
                num_thread: 1,
            };

            //copying the value because compute_mosaic consumes the structure
            let output_file = opts.output.clone();
            compute_mosaic(opts);

            let reference : RgbImage = ImageReader::open("tests/reference/ground-truth-kit.png".to_string()).unwrap().decode().unwrap().into_rgb8();
            let output : RgbImage = ImageReader::open(&output_file).unwrap().decode().unwrap().into_rgb8();

            //verify step by step, first presence, then format, then content
            assert!(std::path::Path::new(&output_file).exists());
            assert_eq!(reference.dimensions(), output.dimensions());
            assert_eq!(reference, output);
        }
    }
