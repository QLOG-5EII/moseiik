#[path = "../src/main.rs"]
mod main; // Importez le module principal

use main::{compute_mosaic, Options}; // Utilisez les éléments du module principal

#[cfg(test)]
mod tests {
    use super::*; // Utilisez les éléments du module principal

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        // TODO: votre implémentation de test ici
        // Définir les options pour l'appel à compute_mosaic
        let options = Options {
	// chemin de l'image source à convertir
            image: "assets/kit.jpeg".to_string(),
	//chemin de l'image de sortie
            output: "assets/output.png".to_string(),
	//chemin du dossier contenant les images de tuiles
            tiles: "assets/images".to_string(),
	//scaling pour ajuster la taille de l'image de sortie
            scaling: 1,
	//taille des tuiles utilisées dans la conversion
            tile_size: 25,
	// Indique que les tuiles utilisées ne doivent pas être supprimées du répertoire source
            remove_used: false,
            verbose: true,
	// Utilisation de SIMD (Single Instruction, Multiple Data) pour optimiser les calculs
            simd: true,
	// Nombre de threads à utiliser pour la conversion
            num_thread: 4,
        };

        // Appelez compute_mosaic avec les options
        compute_mosaic(options);

        // Comparez l'image générée avec la vérité terrain
        let generated_image = image::open("assets/output.png").expect("Failed to open generated image");
        let truth_image = image::open("assets/ground-truth-kit-x86.png").expect("Failed to open truth image");
        let res = &truth_image == &generated_image;
	println!("resultat : {}", res);
        // Comparez les deux images 
        //assert_eq!(generated_image, truth_image);
        assert!(res);

    }
// on fait la meme chose pour le test sur arm en utilisant une image réelle "truth_image" pour ARM
    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        // TODO: votre implémentation de test ici
        // Définir les options pour l'appel à compute_mosaic
        let options = Options {
            image: "assets/kit.jpeg".to_string(),
            output: "assets/output.png".to_string(),
            tiles: "assets/images".to_string(),
            scaling: 1,
            tile_size: 25,
            remove_used: false,
            verbose: true,
            simd: true,
            num_thread: 4,
        };

        // Appelez compute_mosaic avec les options
        compute_mosaic(options);

        // Comparez l'image générée avec la vérité terrain
        let generated_image = image::open("assets/output.png").expect("Failed to open generated image");
        let truth_image = image::open("assets/ground-truth-kit.png").expect("Failed to open truth image");
        let res = &truth_image == &generated_image;
	println!("resultat : {}", res);
        // Comparez les deux images 
        //assert_eq!(generated_image, truth_image);
        assert!(res);

    }

}
