#[cfg(test)]
mod test {
    use crate::{compress, decompress};
    use std::str::FromStr;

    const LOREM_IPSUM : &[u8] = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit. In tellus elit, tristique vitae mattis egestas, ultricies vitae risus. Quisque sit amet quam ut urna aliquet
molestie. Proin blandit ornare dui, a tempor nisl accumsan in. Praesent a consequat felis. Morbi metus diam, auctor in auctor vel, feugiat id odio. Curabitur ex ex,
dictum quis auctor quis, suscipit id lorem. Aliquam vestibulum dolor nec enim vehicula, porta tristique augue tincidunt. Vivamus ut gravida est. Sed pellentesque, dolor
vitae tristique consectetur, neque lectus pulvinar dui, sed feugiat purus diam id lectus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per


inceptos himenaeos. Maecenas feugiat velit in ex ultrices scelerisque id id neque.
";

    #[test]
    fn end_to_end() {
        let input_filename = format!("tests/{}", uuid::Uuid::new_v4());

        std::fs::write(input_filename.clone(), LOREM_IPSUM).expect("Unable to write file");

        let compress_output_filename = format!("tests/{}", uuid::Uuid::new_v4());
        let decompress_output_filename = format!("tests/{}", uuid::Uuid::new_v4());
        unsafe {
            compress(
                std::path::PathBuf::from_str(input_filename.as_str()).unwrap(),
                std::path::PathBuf::from_str(compress_output_filename.as_str()).unwrap(),
            );

            decompress(
                std::path::PathBuf::from_str(compress_output_filename.as_str()).unwrap(),
                std::path::PathBuf::from_str(decompress_output_filename.as_str()).unwrap(),
            );
        }

        let data = std::fs::read(decompress_output_filename).expect("Unable to read file");

        assert_eq!(data, LOREM_IPSUM);
    }
}
