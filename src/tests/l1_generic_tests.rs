use crate::main::*;
use image::RgbImage;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_l1_generic_identical_images() {
        // Crée une image de 2x2 pixels
        let mut im1 = RgbImage::new(2, 2);

        // Remplir les deux images avec la même couleur
        let color = image::Rgb([100, 150, 200]);
        for pixel in im1.pixels_mut() {
            *pixel = color;
        }

        let im2 = im1.clone();

        // La différence attendue doit être 0, car les images sont identiques
        let expected_diff = 0;
        let result = l1_generic(&im1, &im2);
        assert_eq!(result, expected_diff, "Test failed for identical images.");
    }

    // Test 2: Test avec deux images totalement différentes
    #[test]
    fn unit_test_l1_generic_completely_different_images() {
        // Crée une image de 2x2 pixels
        let mut im1 = RgbImage::new(2, 2);
        let mut im2 = RgbImage::new(2, 2);

        // Remplir im1 avec une couleur
        let color1 = image::Rgb([100, 150, 200]);
        for pixel in im1.pixels_mut() {
            *pixel = color1;
        }

        // Remplir im2 avec une couleur différente
        let color2 = image::Rgb([50, 100, 150]);
        for pixel in im2.pixels_mut() {
            *pixel = color2;
        }

        // Calcul de la différence pour chaque pixel :
        // Pour chaque pixel, la différence absolue est :
        // Rouge: |100 - 50| = 50
        // Vert: |150 - 100| = 50
        // Bleu: |200 - 150| = 50
        // Donc pour 4 pixels : 50 + 50 + 50 = 150 (par pixel), multiplié par 4 pixels = 600
        let expected_diff = 600;
        let result = l1_generic(&im1, &im2);
        assert_eq!(
            result, expected_diff,
            "Test failed for completely different images."
        );
    }

    // Test 3: Test avec des images légèrement différentes
    #[test]
    fn unit_test_l1_generic_slightly_different_images() {
        // Crée une image de 2x2 pixels
        let mut im1 = RgbImage::new(2, 2);

        // Remplir im1 avec une couleur
        let color1 = image::Rgb([100, 150, 200]);
        for pixel in im1.pixels_mut() {
            *pixel = color1;
        }

        let color2 = image::Rgb([110, 150, 200]); // Seul le rouge change
        let mut im2 = im1.clone();
        im2.put_pixel(0, 0, color2);

        // La différence attendue :
        // Pour le pixel modifié :
        // Rouge: |110 - 100| = 10
        // Vert: |150 - 150| = 0
        // Bleu: |200 - 200| = 0
        // Différe|nce totale = 10 + 0 + 0 + (3 autres pixels identiques = 0) = 10
        let expected_diff = 10;
        let result = l1_generic(&im1, &im2);
        assert_eq!(
            result, expected_diff,
            "Test failed for slightly different images."
        );
    }

    // Test 4: Test avec des images de taille 1x1
    #[test]
    fn unit_test_l1_generic_single_pixel() {
        // Crée deux images de 1x1 pixel
        let mut im1 = RgbImage::new(1, 1);
        let mut im2 = RgbImage::new(1, 1);

        // Remplir im1 avec une couleur
        let color1 = image::Rgb([100, 150, 200]);
        im1.put_pixel(0, 0, color1);

        // Remplir im2 avec une couleur différente
        let color2 = image::Rgb([50, 100, 150]);
        im2.put_pixel(0, 0, color2);

        // Calcul de la différence :
        // Rouge: |100 - 50| = 50
        // Vert: |150 - 100| = 50
        // Bleu: |200 - 150| = 50
        // Différence totale = 50 + 50 + 50 = 150
        let expected_diff = 150;
        let result = l1_generic(&im1, &im2);
        assert_eq!(
            result, expected_diff,
            "Test failed for single pixel images."
        );
    }
}