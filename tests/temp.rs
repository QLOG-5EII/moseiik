#[cfg(test)]
mod tests {
    use moseiik::main::compute_mosaic;
    use moseiik::main::hash_image;
    use moseiik::main::Options;
    use image::io::Reader as ImageReader;
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        compute_mosaic(Options {
       image: String::from("assets/kit.jpeg"),
       output: String::from("outputCi.png"),
       tiles: String::from("assets/tiles-small/images"),
       scaling: 1,
       tile_size: 25,
       remove_used: false,
       verbose: true,
       simd: false,
       num_thread: 1,
   });
   let img = ImageReader::open("outputCi.png")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();
    let hashOutput = hash_image(&img);

    let imggt = ImageReader::open("assets/ground-truth-kit.png")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();
    let hashGroundTruth = hash_image(&imggt);
    assert_eq!(hashOutput, hashGroundTruth);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
       compute_mosaic(Options {
       image: String::from("assets/kit.jpeg"),
       output: String::from("outputCi.png"),
       tiles: String::from("assets/tiles-small/images"),
       scaling: 1,
       tile_size: 25,
       remove_used: false,
       verbose: true,
       simd: false,
       num_thread: 1,
   });
   let img = ImageReader::open("outputCi.png")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();
    let hashOutput = hash_image(&img);

    let imggt = ImageReader::open("assets/ground-truth-kit.png")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();
    let hashGroundTruth = hash_image(&imggt);
    assert_eq!(hashOutput, hashGroundTruth);
    }

    #[test]
    fn test_generic() {
        compute_mosaic(Options {
       image: String::from("assets/kit.jpeg"),
       output: String::from("outputGeneric.png"),
       tiles: String::from("assets/tiles-small/images"),
       scaling: 1,
       tile_size: 25,
       remove_used: false,
       verbose: true,
       simd: false,
       num_thread: 1,
   });
   let img = ImageReader::open("outputGeneric.png")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();
    let hashOutput = hash_image(&img);

    let imggt = ImageReader::open("assets/ground-truth-kit.png")
        .unwrap()
        .decode()
        .unwrap()
        .into_rgb8();
    let hashGroundTruth = hash_image(&imggt);
    assert_eq!(hashOutput, hashGroundTruth);
    }
}
