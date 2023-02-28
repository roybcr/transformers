mod transformer;

use transformer::{ByteTransformer, Transform};

fn main() {
      let bytes_transformer: &'static ByteTransformer = &ByteTransformer {};
      let bytes: Vec<&'static [u8]> = vec![b"happy new year!", b"hello world"];

      println!(
            "{:#?}, {:#?}",
            bytes_to_string(&bytes_transformer, bytes.clone()),
            bytes
      );
}

pub fn bytes_to_string(
      transformer: &'static ByteTransformer,
      bytes: Vec<&'static [u8]>,
) -> Vec<String> {
      transformer.map(|t| t.to_string())(bytes)
}
