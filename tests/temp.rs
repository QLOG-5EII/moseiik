#[cfg(test)]
mod tests {
    use moseiik::main::*;
    use image::{
        io::Reader as ImageReader,
    };

    /**
     * TESTS D'INTÉGRATION
     * Nous devions faire les tests d'intégration sur la base d'images fournie. 
     * Or, la base d'image étant lourde, nous ne voulions ni la mettre sur le GitHub, ni la télécharger 
     * à chaque éxécution sur GitHUb Action. Nous avons testé localement les tests d'intégration sur la base
     * ceux-ci fonctionnaient. Afin de les exécuter sur GitHub Actions, on a lancé les tests de la fonction
     * compute_mosaic sur les small-tiles. 
     */

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        
        let o = Options{
            image:String::from("assets/target-small.png"),
            output:String::from("assets/result-x86.png"),
            tiles:String::from("assets/tiles-small"),
            scaling: 1,
            tile_size: 5,
            remove_used:false,
            verbose:true,
            simd:true,
            num_thread:1,
        };
        compute_mosaic(o);
        let img1 = ImageReader::open("./assets/target-small.png").unwrap().decode().unwrap().to_rgb8();
        let img2 = ImageReader::open("./assets/result-x86.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(img1, img2);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        let o = Options{
            image:String::from("assets/target-small.png"),
            output:String::from("assets/result-arm.png"),
            tiles:String::from("assets/tiles-small"),
            scaling: 1,
            tile_size: 5,
            remove_used:false,
            verbose:false,
            simd:true,
            num_thread:1,
        };
        compute_mosaic(o);
        let img1 = ImageReader::open("./assets/target-small.png").unwrap().decode().unwrap().to_rgb8();
        let img2 = ImageReader::open("./assets/result-arm.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(img1, img2);
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_generic() {
        let o = Options{
            image:String::from("assets/target-small.png"),
            output:String::from("assets/result-generic.png"),
            tiles:String::from("assets/tiles-small"),
            scaling: 1,
            tile_size: 5,
            remove_used:false,
            verbose:true,
            simd:false,
            num_thread:1,
        };
        compute_mosaic(o);
        let img1 = ImageReader::open("./assets/target-small.png").unwrap().decode().unwrap().to_rgb8();
        let img2 = ImageReader::open("./assets/result-generic.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(img1, img2);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_generic() {
        let o = Options{
            image:String::from("assets/target-small.png"),
            output:String::from("assets/result-arm.png"),
            tiles:String::from("assets/tiles-small"),
            scaling: 1,
            tile_size: 5,
            remove_used:false,
            verbose:false,
            simd:false,
            num_thread:1,
        };
        compute_mosaic(o);
        let img1 = ImageReader::open("./assets/target-small.png").unwrap().decode().unwrap().to_rgb8();
        let img2 = ImageReader::open("./assets/result-arm.png").unwrap().decode().unwrap().to_rgb8();
        assert_eq!(img1, img2);
    }
}
