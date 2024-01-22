#[cfg(test)]
mod tests {
    use moseiik::main::Options;
    use moseiik::main::compute_mosaic;
    use moseiik::main::load_image;


    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
    
  	//image de référence

       let options = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/Real_one.png"),
        tiles: String::from("./assets/images"),
        scaling: 1,  
        tile_size: 25, 
        remove_used: false, 
        verbose: false,  
        simd: true,  
        num_thread: 4,  
    };
    compute_mosaic(options);

    // Creation options SIMD variable
    let options_simd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 1,  
        tile_size: 25,
        remove_used: false,
        verbose: false,  
        simd: true,  
        num_thread: 4,  
    };

    let options_nsimd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 1,  
        tile_size: 25,  
        remove_used: false, 
        verbose: false,  
        simd: false, 
        num_thread: 4,  
    };
    
    //Test SIMD
    compute_mosaic(options_simd);
    let image_res = load_image("./assets/test_mosaic.png");
    let image_init = load_image("./assets/Real_one.png");
    assert_eq!(image_res, image_init);
    
    //Test Non_SIMD
    compute_mosaic(options_nsimd);
    let image_res = load_image("./assets/test_mosaic.png");
    let image_init = load_image("./assets/Real_one.png");
    assert_eq!(image_res, image_init);
    
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
 // Creation image référence

       let options = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/Real_one.png"),
        tiles: String::from("./assets/images"),
        scaling: 1, 
        tile_size: 25,  
        remove_used: false,  
        verbose: false, 
        simd: true,  
        num_thread: 4,  
    };
    compute_mosaic(options);

    // Creation options SIMD variable
    let options_simd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 1,  
        tile_size: 25,  
        remove_used: false,  
        verbose: false,  
        simd: true,  
        num_thread: 4,  
    };

    let options_nsimd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 1,  
        tile_size: 25, 
        remove_used: false,  
        verbose: false,  
        simd: false, 
        num_thread: 4,  
    };
    
    //Test SIMD
    compute_mosaic(options_simd);
    let image_res = load_image("./assets/test_mosaic.png");
    let image_init = load_image("./assets/Real_one.png");
    assert_eq!(image_res, image_init);
    
    //Test Non_SIMD
    compute_mosaic(options_nsimd);
    let image_res = load_image("./assets/test_mosaic.png");
    let image_init = load_image("./assets/Real_one.png");
    assert_eq!(image_res, image_init);
    }

    #[test]
    fn test_generic() {
 // Creation image référence

       let options = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/Real_one.png"),
        tiles: String::from("./assets/images"),
        scaling: 1,  
        tile_size: 25,  
        remove_used: false,  
        verbose: false, 
        simd: true,  
        num_thread: 4,  
    };
    compute_mosaic(options);

    // Creation options SIMD variable
    let options_simd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 2,  
        tile_size: 25, 
        remove_used: false,  
        verbose: false,  
        simd: true, 
        num_thread: 4,  
    };

    let options_nsimd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 1,  
        tile_size: 25,
        remove_used: false, 
        verbose: false,  
        simd: false,  
        num_thread: 4, 
    };
    
    //Test SIMD
    compute_mosaic(options_simd);
    let image_res = load_image("./assets/test_mosaic.png");
    let image_init = load_image("./assets/Real_one.png");
    assert_eq!(image_res, image_init);
    
    //Test Non_SIMD
    compute_mosaic(options_nsimd);
    let image_res = load_image("./assets/test_mosaic.png");
    let image_init = load_image("./assets/Real_one.png");
    assert_eq!(image_res, image_init);
    }
}
