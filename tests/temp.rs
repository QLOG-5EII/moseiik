#[cfg(test)]
mod tests {
    use moseiik::main::{compute_mosaic, Options};
    use image::io::Reader as ImageReader;
    use image::RgbImage;
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        let opt = Options{
            image: "assets/kit.png".to_string(),
            output: "tests/kit_test.png".to_string(),
            tiles: "assets/images".to_string(),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: true,
            num_thread: 1,
        };
        let reference : RgbImage = ImageReader::open("assets/ground-truth-kit.png".to_string()).unwrap().decode().unwrap().into_rgb8();
        let output : RgbImage = ImageReader::open("tests/kit_test.png".to_string()).unwrap().decode().unwrap().into_rgb8();
        assert_eq!(reference,output);
    }
    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        let opt = Options{
            image: "assets/kit.png".to_string(),
            output: "tests/kit_test.png".to_string(),
            tiles: "assets/images".to_string(),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: true,
            num_thread: 1,
        };
        let reference : RgbImage = ImageReader::open("assets/ground-truth-kit.png".to_string()).unwrap().decode().unwrap().into_rgb8();
        let output : RgbImage = ImageReader::open("tests/kit_test.png".to_string()).unwrap().decode().unwrap().into_rgb8();
        assert_eq!(reference,output);
    }
    #[test]
    fn test_generic() {
        let opt = Options{
            image: "assets/kit.png".to_string(),
            output: "tests/kit_test.png".to_string(),
            tiles: "assets/images".to_string(),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: false,
            num_thread: 1,
        };
        let reference : RgbImage = ImageReader::open("assets/ground-truth-kit.png".to_string()).unwrap().decode().unwrap().into_rgb8();
        let output : RgbImage = ImageReader::open("tests/kit_test.png".to_string()).unwrap().decode().unwrap().into_rgb8();
        assert_eq!(reference,output);
    }
}
