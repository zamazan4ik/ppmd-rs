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
        let reference_filename = "tests/large_compressed.txt.ppmd";
        let input_filename = "tests/large_uncompressed.txt";
        let output_filename = format!("tests/generated/{}", uuid::Uuid::new_v4().to_string());
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
        let reference_filename = "tests/large_uncompressed.txt";
        let input_filename = "tests/large_compressed.txt.ppmd";
        let output_filename = format!("tests/generated/{}", uuid::Uuid::new_v4().to_string());
        unsafe {
            decompress(
                std::path::PathBuf::from_str(input_filename).unwrap(),
                std::path::PathBuf::from_str(output_filename.as_str()).unwrap(),
            );
        }

        assert!(compare_files(output_filename.as_str(), reference_filename));
    }
}
