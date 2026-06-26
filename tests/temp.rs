#[cfg(test)]
mod tests {
    use image::io::Reader as ImageReader;
    use std::fs;
    use std::process::Command;

    
    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
        let output_path = "out-test-x86.png";

        let status = Command::new("cargo")
            .args([
                "run",
                "--release",
                "--",
                "--image",
                "assets/target-small.png",
                "--tiles",
                "assets/tiles-small",
                "--output",
                output_path,
                "--tile-size",
                "5",
                "--num-thread",
                "2",
                "--simd",
            ])
            .status()
            .expect("failed to run moseiik with simd");

        assert!(status.success());

        let target = ImageReader::open("assets/target-small.png")
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();

        let output = ImageReader::open(output_path)
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();

        assert_eq!(target, output);

        let _ = fs::remove_file(output_path);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        let output_path = "out-test-aarch64.png";

        let status = Command::new("cargo")
            .args([
                "run",
                "--release",
                "--",
                "--image",
                "assets/target-small.png",
                "--tiles",
                "assets/tiles-small",
                "--output",
                output_path,
                "--tile-size",
                "5",
                "--num-thread",
                "2",
                "--simd",
            ])
            .status()
            .expect("failed to run moseiik with simd");

        assert!(status.success());

        let target = ImageReader::open("assets/target-small.png")
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();

        let output = ImageReader::open(output_path)
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();

        assert_eq!(target, output);

        let _ = fs::remove_file(output_path);
    }

    #[test]
    fn test_generic() {
        let output_path = "out-test-generic.png";

        let status = Command::new("cargo")
            .args([
                "run",
                "--release",
                "--",
                "--image",
                "assets/target-small.png",
                "--tiles",
                "assets/tiles-small",
                "--output",
                output_path,
                "--tile-size",
                "5",
                "--num-thread",
                "1",
            ])
            .status()
            .expect("failed to run moseiik");

        assert!(status.success());

        let target = ImageReader::open("assets/target-small.png")
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();

        let output = ImageReader::open(output_path)
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();

        assert_eq!(target, output);

        let _ = fs::remove_file(output_path);
    }
}
