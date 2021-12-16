use crate::dec_double::DECBYTES;

/* Top words for a zero                                           */
pub const SINGLE_ZERO: u32 = 0x22500000;
pub const DOUBLE_ZERO: u32 = 0x22380000;
pub const QUAD_ZERO: u32 = 0x22080000;

pub const ZERO_WORD: u32 = QUAD_ZERO;

pub const DECWORDS: usize = (DECBYTES / 4) as usize;

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
//   use crate::dec_quad::DecQuad;
//
//   #[test]
//   fn test_df_word() {
//     let dq = DecQuad::default();
//     //df_word!(dq, 0);
//   }
// }
