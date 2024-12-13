use crate::main::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_prepare_tiles() {
        //Unit Test pour la fonction Prepare_tiles

        //Set different taille de tiles
        let tile_sizes = vec![
            Size {
                width: 30,
                height: 25,
            },
            Size {
                width: 40,
                height: 40,
            },
            Size {
                width: 50,
                height: 30,
            },
        ];

        let tiles_path = "./assets/tiles-small/"; // Chemain d'acces des tiles
        let verbose = false; // desactive la verbose

        for tile_size in tile_sizes {
            let tiles = &prepare_tiles(tiles_path, &tile_size, verbose).unwrap();

            for (i, tile) in tiles.iter().enumerate() {
                // Test si la tile à les dimensions voulue
                assert_eq!(
                    tile.width(),
                    tile_size.width,
                    "Tile {} has incorrect width for size {:?}",
                    i,
                    tile_size
                );
                assert_eq!(
                    tile.height(),
                    tile_size.height,
                    "Tile {} has incorrect height for size {:?}",
                    i,
                    tile_size
                );
            }
        }
    }

    #[test]
    fn test_prepare_tiles_invalid_folder() {
        let invalid_path = "./non_existent_folder";

        let result = prepare_tiles(
            invalid_path,
            &Size {
                width: 30,
                height: 30,
            },
            false,
        );
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_prepare_tiles_invalid_size() {
        let invalid_path = "./assets/tiles-small/";

        //Both size = 0
        let result = prepare_tiles(
            invalid_path,
            &Size {
                width: 0,
                height: 0,
            },
            false,
        );
        assert_eq!(result.is_err(), true);

        //Width > 0 Height = 0
        let result = prepare_tiles(
            invalid_path,
            &Size {
                width: 10,
                height: 0,
            },
            false,
        );
        assert_eq!(result.is_err(), true);

        //Width = 0 Height > 0
        let result = prepare_tiles(
            invalid_path,
            &Size {
                width: 0,
                height: 10,
            },
            false,
        );
        assert_eq!(result.is_err(), true);
    }
}
