pub const DEC_QUAD_ZERO: u32 = 0x22080000;

#[macro_export]
macro_rules! exp_is_special {
  ($exp:expr) => {
    $exp >= DECFLOAT_MIN_SP
  };
}

// #[macro_export]
// macro_rules! df_word {
//   ($df:expr, $off:expr) => {
//     unsafe { $df.words[DECWORDS - 1 - $off] }
//   };
// }
//
// //DFWORD(df, off)   ((df)->words[DECWORDS-1-(off)])
//
// #[cfg(test)]
// mod tests {
//   use super::*;
//   use crate::dec_quad::{DECFLOAT_MIN_SP, DecQuad};
//
//   #[test]
//   fn test_df_word() {
//     let dq = DecQuad::default();
//     //df_word!(dq, 0);
//   }
// }
