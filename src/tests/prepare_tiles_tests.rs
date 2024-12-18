/* 
 * Project   : moseiik
 * Authors   : Faucheux Valentin and Plumejeau Maxime
 * File      : unit testing of prepare_tiles function
 * Comments  : verfiy correct behavior of prepare_tiles function for the following cases :
 *             - 3 different tile sizes
 *             - invalid folder path
 *             - invalid tile size
 */
use crate::main::*;

#[cfg(test)]
mod tests {
    use super::*;

    /*
     * Test 1: Verify that 'prepare_tiles' returns the correct size of tiles
     * for 3 different input tile sizes
     */
    #[test]
    fn unit_test_prepare_tiles() {

        //Set tile sizes
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
        // tiles path
        let tiles_path = "./assets/tiles-small/";
        let verbose = false; // verbose mode

        for tile_size in tile_sizes {
            let tiles = &prepare_tiles(tiles_path, &tile_size, verbose).unwrap();

            for (i, tile) in tiles.iter().enumerate() {
                // Test if the tile has the correct size
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

    /*
     * Test 2: Verify that 'prepare_tiles' returns an error for an invalid folder path
     */
    #[test]
    fn test_prepare_tiles_invalid_folder() {
        // invalid folder path
        let invalid_path = "./non_existent_folder";

        let result = prepare_tiles(
            invalid_path,
            &Size {
                width: 30,
                height: 30,
            },
            false,
        );
        // Test if the function returns an error
        assert_eq!(result.is_err(), true);
    }

    /*
     * Test 3: Verify that 'prepare_tiles' returns an error for invalid tile sizes
     */
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
