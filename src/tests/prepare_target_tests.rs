/* 
 * Project   : moseiik
 * Authors   : Faucheux Valentin and Plumejeau Maxime
 * File      : unit testing of prepare_target function
 * Comments  : verfiy correct behavior of prepare_target function for the following cases :
 *             - Different image sizes
 *             - Different scalings
 */

use crate::main::*;
use image::ImageReader;

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * Test 1: Verify that 'prepare_target' returns an image with the correct dimensions
     * for different image sizes and scalings
     */
    #[test]
    fn unit_test_prepare_target() {
        use std::path::Path;

        // Function to calculate the expected output dimensions
        fn calculate_output_dimensions(
            image_width: u32,
            image_height: u32,
            scaling: u32,
            size: &Size,
        ) -> (u32, u32) {
            let output_image_width = image_width * scaling - (image_width * scaling) % size.width;
            let output_image_height =
                image_height * scaling - (image_height * scaling) % size.height;
            (output_image_width, output_image_height)
        }

        // Test cases
        let test_cases = vec![
            (
                "./assets/kit.jpeg",
                1,
                Size {
                    width: 20,
                    height: 20,
                },
            ),
            (
                "./assets/kit.jpeg",
                3,
                Size {
                    width: 20,
                    height: 20,
                },
            ),
            (
                "./assets/target-small.png",
                1,
                Size {
                    width: 25,
                    height: 25,
                },
            ),
        ];

        for (image_path, scaling, tile_size) in test_cases {
            // Check if the image file exists
            assert!(
                Path::new(image_path).exists(),
                "Le fichier image {} est introuvable.",
                image_path
            );

            // Load the target image and get its dimensions
            let target = ImageReader::open(image_path)
                .expect("Impossible d'ouvrir l'image")
                .decode()
                .expect("Impossible de décoder l'image")
                .into_rgb8();

            let image_width = target.width();
            let image_height = target.height();

            let (expected_width, expected_height) =
                calculate_output_dimensions(image_width, image_height, scaling, &tile_size);

            // Test the prepare_target function
            match prepare_target(image_path, scaling, &tile_size) {
                Ok(result) => {
                    let result_size = Size {
                        width: result.width(),
                        height: result.height(),
                    };
                    assert_eq!(
                        result_size.width, expected_width,
                        "Largeur incorrecte pour {} avec scaling {}.",
                        image_path, scaling
                    );
                    assert_eq!(
                        result_size.height, expected_height,
                        "Hauteur incorrecte pour {} avec scaling {}.",
                        image_path, scaling
                    );
                }
                Err(e) => panic!(
                    "Erreur lors de la préparation de l'image {}: {}",
                    image_path, e
                ),
            }
        }
    }
}
