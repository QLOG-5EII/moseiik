#[cfg(test)]
mod tests {
    use moseiik::main::compute_mosaic;
    use moseiik::main::Options;
    use moseiik::main::load_image_from_path;
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        // Creation image référence
       let options = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/Real_one.png"),
        tiles: String::from("./assets/images"),
        scaling: 2,  // Facteur de mise à l'échelle
        tile_size: 25,  // Taille des vignettes
        remove_used: false,  // Supprimer ou non les vignettes utilisées
        verbose: false,  // Activer ou non les logs détaillés
        simd: true,  // Utiliser SIMD si disponible
        num_thread: 4,  // Nombre de threads à utiliser
    };
    compute_mosaic(options);

    // Creation options SIMD variable
    let options_simd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 2,  // Facteur de mise à l'échelle
        tile_size: 25,  // Taille des vignettes
        remove_used: false,  // Supprimer ou non les vignettes utilisées
        verbose: false,  // Activer ou non les logs détaillés
        simd: true,  // Utiliser SIMD si disponible
        num_thread: 4,  // Nombre de threads à utiliser
    };

    let options_nsimd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 2,  // Facteur de mise à l'échelle
        tile_size: 25,  // Taille des vignettes
        remove_used: false,  // Supprimer ou non les vignettes utilisées
        verbose: false,  // Activer ou non les logs détaillés
        simd: false,  // Utiliser SIMD si disponible
        num_thread: 4,  // Nombre de threads à utiliser
    };
    
    //Test SIMD
    compute_mosaic(options_simd);
    let image_res = load_image_from_path("./assets/test_mosaic.png");
    let image_init = load_image_from_path("./assets/Real_one.png");
    assert_eq!(image_res, image_init);
    
    //Test Non_SIMD
    compute_mosaic(options_nsimd);
    let image_res = load_image_from_path("./assets/test_mosaic.png");
    let image_init = load_image_from_path("./assets/Real_one.png");
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
        scaling: 2,  // Facteur de mise à l'échelle
        tile_size: 25,  // Taille des vignettes
        remove_used: false,  // Supprimer ou non les vignettes utilisées
        verbose: false,  // Activer ou non les logs détaillés
        simd: true,  // Utiliser SIMD si disponible
        num_thread: 4,  // Nombre de threads à utiliser
    };
    compute_mosaic(options);

    // Creation options SIMD variable
    let options_simd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 2,  // Facteur de mise à l'échelle
        tile_size: 25,  // Taille des vignettes
        remove_used: false,  // Supprimer ou non les vignettes utilisées
        verbose: false,  // Activer ou non les logs détaillés
        simd: true,  // Utiliser SIMD si disponible
        num_thread: 4,  // Nombre de threads à utiliser
    };

    let options_nsimd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 2,  // Facteur de mise à l'échelle
        tile_size: 25,  // Taille des vignettes
        remove_used: false,  // Supprimer ou non les vignettes utilisées
        verbose: false,  // Activer ou non les logs détaillés
        simd: false,  // Utiliser SIMD si disponible
        num_thread: 4,  // Nombre de threads à utiliser
    };
    
    //Test SIMD
    compute_mosaic(options_simd);
    let image_res = load_image_from_path("./assets/test_mosaic.png");
    let image_init = load_image_from_path("./assets/Real_one.png");
    assert_eq!(image_res, image_init);
    
    //Test Non_SIMD
    compute_mosaic(options_nsimd);
    let image_res = load_image_from_path("./assets/test_mosaic.png");
    let image_init = load_image_from_path("./assets/Real_one.png");
    assert_eq!(image_res, image_init);
    }

    #[test]
    fn test_generic() {
        // Creation image référence

       let options = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/Real_one.png"),
        tiles: String::from("./assets/images"),
        scaling: 2,  // Facteur de mise à l'échelle
        tile_size: 25,  // Taille des vignettes
        remove_used: false,  // Supprimer ou non les vignettes utilisées
        verbose: false,  // Activer ou non les logs détaillés
        simd: true,  // Utiliser SIMD si disponible
        num_thread: 4,  // Nombre de threads à utiliser
    };
    compute_mosaic(options);

    // Creation options SIMD variable
    let options_simd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 2,  // Facteur de mise à l'échelle
        tile_size: 25,  // Taille des vignettes
        remove_used: false,  // Supprimer ou non les vignettes utilisées
        verbose: false,  // Activer ou non les logs détaillés
        simd: true,  // Utiliser SIMD si disponible
        num_thread: 4,  // Nombre de threads à utiliser
    };

    let options_nsimd = Options {
        image: String::from("./assets/kit.jpeg"),
        output: String::from("./assets/test_mosaic.png"),
        tiles: String::from("./assets/images"),
        scaling: 2,  // Facteur de mise à l'échelle
        tile_size: 25,  // Taille des vignettes
        remove_used: false,  // Supprimer ou non les vignettes utilisées
        verbose: false,  // Activer ou non les logs détaillés
        simd: false,  // Utiliser SIMD si disponible
        num_thread: 4,  // Nombre de threads à utiliser
    };
    
    //Test SIMD
    compute_mosaic(options_simd);
    let image_res = load_image_from_path("./assets/test_mosaic.png");
    let image_init = load_image_from_path("./assets/Real_one.png");
    assert_eq!(image_res, image_init);
    
    //Test Non_SIMD
    compute_mosaic(options_nsimd);
    let image_res = load_image_from_path("./assets/test_mosaic.png");
    let image_init = load_image_from_path("./assets/Real_one.png");
    assert_eq!(image_res, image_init);
    }
}
