#[cfg(test)]
mod tests {

    use image::io::Reader as ImageReader;
    use moseiik::{compute_mosaic, Options};
    use std::fs;

    fn run_integration_test(use_simd: bool, output_filename: &str) {
        let options = Options {
            image: "assets/kit.jpeg".to_string(),
            output: output_filename.to_string(),
            tiles: "assets/images".to_string(),
            scaling: 1,
            tile_size: 5,
            remove_used: false,
            verbose: false,
            simd: use_simd,
            num_thread: 1,
        };

        // Exécution du processus
        compute_mosaic(options);

        // Chargement des images pour comparaison
        let generated_image = ImageReader::open(output_filename)
            .expect("Impossible d'ouvrir l'image générée")
            .decode()
            .unwrap()
            .into_rgb8();

        let refered_image = ImageReader::open("assets/out.png")
            .expect("Impossible d'ouvrir l'image de référence")
            .decode()
            .unwrap()
            .into_rgb8();

        // Vérification image generer vs image de reference
        assert!(
            generated_image == refered_image,
            "L'image générée ne correspond pas à l'image de référence !"
        );

        // Nettoyage du fichier temporaire
        fs::remove_file(output_filename).unwrap();
    }

    // --- l1_x86_sse2 --------------------------------------------------------
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        // Teste le rendu global en utilisant les instructions SIMD SSE2
        run_integration_test(true, "assets/out_x86_test.png");
    }

    // --- l1_neon ------------------------------------------------------------
    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        // Teste le rendu global en utilisant les instructions SIMD NEON
        run_integration_test(true, "assets/out_aarch64_test.png");
    }

    // --- l1_generic ---------------------------------------------------------
    #[test]
    fn test_generic() {
        // Teste le rendu global avec la fonction de calcul de distance générique
        run_integration_test(false, "assets/out_generic_test.png");
    }
}
