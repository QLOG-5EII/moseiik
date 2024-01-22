#[cfg(test)]
mod tests {
    use moseiik::main::{compute_mosaic, Options};
    use image::{
        imageops::{resize, FilterType::Nearest},
        io::Reader as ImageReader,
        GenericImage, GenericImageView, RgbImage,
    };
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        let ArgumentsSimd = Options{
            image: String ::from("assets/kit.jpeg"),
            output: String::from("assets/result_x86.png"),
            tiles: String::from("assets/images/"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: true,
            num_thread: 4,
        };
        let Arguments = Options{
            image: String ::from("assets/kit.jpeg"),
            output: String::from("assets/result_x86.png"),
            tiles: String::from("assets/images/"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: false,
            num_thread: 4,
        };


        compute_mosaic(Arguments);
        let img1= ImageReader::open("./assets/ground-truth-kit_x86.png").unwrap().decode().unwrap().to_rgb8();
        let result= ImageReader::open("./assets/result_x86.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(result,img1); // on vient tester si les images sont les mêmes

        compute_mosaic(ArgumentsSimd);
        let img1= ImageReader::open("./assets/ground-truth-kit_x86.png").unwrap().decode().unwrap().to_rgb8();
        let result= ImageReader::open("./assets/result_x86.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(result,img1); // on vient tester si les images sont les mêmes


    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        let ArgumentsSimd = Options{
            image: String ::from("assets/kit.jpeg"),
            output: String::from("assets/result_x86.png"),
            tiles: String::from("assets/images/"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: true,
            num_thread: 4,
        };
        let Arguments = Options{
            image: String ::from("assets/kit.jpeg"),
            output: String::from("assets/result_x86.png"),
            tiles: String::from("assets/images/"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: false,
            num_thread: 4,
        };

        compute_mosaic(Arguments);
        let img1= ImageReader::open("./assets/ground-truth-kit_aarch64.png").unwrap().decode().unwrap().to_rgb8();
        let result= ImageReader::open("./assets/result_aarch64.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(result,img1); // on vient tester si les images sont les mêmes

        compute_mosaic(ArgumentsSimd);
        let img1= ImageReader::open("./assets/ground-truth-kit_aarch64.png").unwrap().decode().unwrap().to_rgb8();
        let result= ImageReader::open("./assets/result_aarch64.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(result,img1); // on vient tester si les images sont les mêmes

    }

    #[test]
    fn test_generic() {
        let Arguments = Options{
            image: String ::from("assets/kit.jpeg"),
            output: String::from("assets/result.png"),
            tiles: String::from("assets/images/"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: false,
            num_thread: 4,
        };
        let ArgumentsSimd = Options{
            image: String ::from("assets/kit.jpeg"),
            output: String::from("assets/result.png"),
            tiles: String::from("assets/images/"),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: false,
            simd: true,
            num_thread: 4,
        };


        compute_mosaic(Arguments);
        let img1= ImageReader::open("./assets/ground-truth-kit.png").unwrap().decode().unwrap().to_rgb8();
        let result= ImageReader::open("./assets/result.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(result,img1); //on test si les images obtenues sont les mêmes

        compute_mosaic(ArgumentsSimd);
        let img1= ImageReader::open("./assets/ground-truth-kit.png").unwrap().decode().unwrap().to_rgb8();
        let result= ImageReader::open("./assets/result.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(result,img1); //on test si les images obtenues sont les mêmes

    }

}
