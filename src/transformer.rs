#![allow(dead_code)]

pub trait Transform {
      type Input;

      fn map<F, X, T>(
            &'static self,
            func: F,
      ) -> Box<(dyn Fn(Vec<Self::Input>) -> Vec<T>)>
      where
            F: Fn(&Self) -> X + 'static,
            X: Fn(Self::Input) -> T,
      {
            return Box::from(
                  move |input: Vec<<Self as Transform>::Input>| -> Vec<T> {
                        input.into_iter()
                              .map(func(&self))
                              .collect()
                  },
            );
      }
}

pub struct ByteTransformer;

impl Transform for ByteTransformer {
      type Input = &'static [u8];
}

impl ByteTransformer {
      pub fn to_string(
            &self,
      ) -> Box<(dyn Fn(<Self as Transform>::Input) -> String)> {
            return Box::from(
                  move |input: <Self as Transform>::Input| -> String {
                        String::from_utf8_lossy(input).into()
                  },
            );
      }
}
