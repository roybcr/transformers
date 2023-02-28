mod transformer;

fn main() {
      let bytes: vec<&'static [u8]> = vec![b"roybach!", b"helworld"];
      let bytes_transformer = ByteTransformer {};
      let x = bytes_transformer.map(|t| t.to_string(), bytes);
      println!("{:#?}", x);

      let y = vec![
            string::from("roybach!"),
            string::from("helworld"),
      ];

      assert_eq!(y, x);
}
