#[cfg(test)]
mod test {
    use crate::{compress, decompress};
    use std::str::FromStr;

    fn compare_files(output: &str, reference: &str) -> bool {
        let mut resulting_file = match std::fs::File::open(output) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };
        let mut reference_file = match std::fs::File::open(reference) {
            Ok(f) => f,
            Err(e) => panic!("{}", e),
        };

        file_diff::diff_files(&mut resulting_file, &mut reference_file)
    }

    #[test]
    fn ppmd8_encode_small() {
        let reference_filename = "tests/large_compressed.png.ppmd";
        let input_filename = "tests/large_uncompressed.png";
        let output_filename = format!("tests/{}", uuid::Uuid::new_v4());
        unsafe {
            compress(
                std::path::PathBuf::from_str(input_filename).unwrap(),
                std::path::PathBuf::from_str(output_filename.as_str()).unwrap(),
            );
        }

        assert!(compare_files(output_filename.as_str(), reference_filename));
    }

    #[test]
    fn ppmd8_decode_small() {
        let reference_filename = "tests/large_uncompressed.png";
        let input_filename = "tests/large_compressed.png.ppmd";
        let output_filename = format!("tests/{}", uuid::Uuid::new_v4());
        unsafe {
            decompress(
                std::path::PathBuf::from_str(input_filename).unwrap(),
                std::path::PathBuf::from_str(output_filename.as_str()).unwrap(),
            );
        }

        assert!(compare_files(output_filename.as_str(), reference_filename));
    }
}
