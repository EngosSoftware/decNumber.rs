/// Rounding modes.
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Rounding {
  /// Round towards `+infinity`.
  DecRoundCeiling = 0,
  /// Round away from 0.
  DecRoundUp = 1,
  /// 0.5 rounds up.
  DecRoundHalfUp = 2,
  /// 0.5 rounds to nearest even.
  DecRoundHalfEven = 3,
  /// 0.5 rounds down.
  DecRoundHalfDown = 4,
  /// Round towards 0 (truncate).
  DecRoundDown = 5,
  /// Round towards `-infinity`.
  DecRoundFloor = 6,
  /// Round for re-round.
  DecRound05up = 7,
}

/// Default rounding mode is [Rounding::DecRoundHalfEven].
pub const DEC_ROUND_DEFAULT: Rounding = Rounding::DecRoundHalfEven;

/// Context for operations.
pub struct DecContext {
  /// Working precision.
  pub digits: i32,
  /// Maximum positive exponent.
  pub emax: i32,
  /// Minimum negative exponent.
  pub emin: i32,
  /// Rounding mode.
  pub round: Rounding,
  /// Trap-enabler flags.
  pub traps: u32,
  /// Status flags.
  pub status: u32,
  /// Flag: apply IEEE exponent clamp.
  pub clamp: u8,
  /// Flag: special-values allowed.
  #[cfg(feature = "dec-subset")]
  pub extended: u8,
}

// Maxima and Minima for context settings.

/// Max digits.
pub const DEC_MAX_DIGITS: i32 = 999999999;
/// Min digits.
pub const DEC_MIN_DIGITS: i32 = 1;
/// Max emax.
pub const DEC_MAX_EMAX: i32 = 999999999;
/// Min emax.
pub const DEC_MIN_EMAX: i32 = 0;
/// Max emin.
pub const DEC_MAX_EMIN: i32 = 0;
/// Min emin.
pub const DEC_MIN_EMIN: i32 = -999999999;
/// Max emax, etc., for mathematical functions.
pub const DEC_MAX_MATH: i32 = 999999;

/// Classifications for decimal numbers, aligned with IEEE 754
/// (note that 'normal' and 'subnormal' are meaningful only with
/// a [DecContext] or a fixed size format.
pub enum DecClass {
  DecClassSNaN = 0,
  DecClassQNaN = 1,
  DecClassNegInf = 2,
  DecClassNegNormal = 3,
  DecClassNegSubnormal = 4,
  DecClassNegZero = 5,
  DecClassPosZero = 6,
  DecClassPosSubnormal = 7,
  DecClassPosNormal = 8,
  DecClassPosInf = 9,
}

// Strings for the DecClasses.

/// String for [DecClass::DecClassSNaN].
pub const DEC_CLASS_STRING_SN: &str = "sNaN";
/// String for [DecClass::DecClassQNaN].
pub const DEC_CLASS_STRING_QN: &str = "NaN";
/// String for [DecClass::DecClassNegInf].
pub const DEC_CLASS_STRING_NI: &str = "-Infinity";
/// String for [DecClass::DecClassNegNormal].
pub const DEC_CLASS_STRING_NN: &str = "-Normal";
/// String for [DecClass::DecClassNegSubnormal].
pub const DEC_CLASS_STRING_NS: &str = "-Subnormal";
/// String for [DecClass::DecClassNegZero].
pub const DEC_CLASS_STRING_NZ: &str = "-Zero";
/// String for [DecClass::DecClassPosZero].
pub const DEC_CLASS_STRING_PZ: &str = "+Zero";
/// String for [DecClass::DecClassPosSubnormal].
pub const DEC_CLASS_STRING_PS: &str = "+Subnormal";
/// String for [DecClass::DecClassPosNormal].
pub const DEC_CLASS_STRING_PN: &str = "+Normal";
/// String for [DecClass::DecClassPosInf].
pub const DEC_CLASS_STRING_PI: &str = "+Infinity";
/// String for invalid value.
pub const DEC_CLASS_STRING_UN: &str = "Invalid";

// Trap-enabler and Status flags (exceptional conditions), and their names.
// The top byte is reserved for internal use.

// Extended flags.
#[cfg(feature = "dec-ext-flag")]
pub const DEC_CONVERSION_SYNTAX: u32 = 0x00000001;
#[cfg(feature = "dec-ext-flag")]
pub const DEC_DIVISION_BY_ZERO: u32 = 0x00000002;
#[cfg(feature = "dec-ext-flag")]
pub const DEC_DIVISION_IMPOSSIBLE: u32 = 0x00000004;
#[cfg(feature = "dec-ext-flag")]
pub const DEC_DIVISION_UNDEFINED: u32 = 0x00000008;
#[cfg(feature = "dec-ext-flag")]
pub const DEC_INSUFFICIENT_STORAGE: u32 = 0x00000010; /* [when malloc fails]  */
#[cfg(feature = "dec-ext-flag")]
pub const DEC_INEXACT: u32 = 0x00000020;
#[cfg(feature = "dec-ext-flag")]
pub const DEC_INVALID_CONTEXT: u32 = 0x00000040;
#[cfg(feature = "dec-ext-flag")]
pub const DEC_INVALID_OPERATION: u32 = 0x00000080;
#[cfg(all(feature = "dec-ext-flag", feature = "dec-subset"))]
pub const DEC_LOST_DIGITS: u32 = 0x00000100;
#[cfg(feature = "dec-ext-flag")]
pub const DEC_OVERFLOW: u32 = 0x00000200;
#[cfg(feature = "dec-ext-flag")]
pub const DEC_CLAMPED: u32 = 0x00000400;
#[cfg(feature = "dec-ext-flag")]
pub const DEC_ROUNDED: u32 = 0x00000800;
#[cfg(feature = "dec-ext-flag")]
pub const DEC_SUBNORMAL: u32 = 0x00001000;
#[cfg(feature = "dec-ext-flag")]
pub const DEC_UNDERFLOW: u32 = 0x00002000;

// IEEE flags only.
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_CONVERSION_SYNTAX: u32 = 0x00000010;
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_DIVISION_BY_ZERO: u32 = 0x00000002;
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_DIVISION_IMPOSSIBLE: u32 = 0x00000010;
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_DIVISION_UNDEFINED: u32 = 0x00000010;
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_INSUFFICIENT_STORAGE: u32 = 0x00000010; /* [when malloc fails]  */
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_INEXACT: u32 = 0x00000001;
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_INVALID_CONTEXT: u32 = 0x00000010;
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_INVALID_OPERATION: u32 = 0x00000010;
#[cfg(all(not(feature = "dec-ext-flag"), feature = "dec-subset"))]
pub const DEC_LOST_DIGITS: u32 = 0x00000000;
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_OVERFLOW: u32 = 0x00000008;
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_CLAMPED: u32 = 0x00000000;
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_ROUNDED: u32 = 0x00000000;
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_SUBNORMAL: u32 = 0x00000000;
#[cfg(not(feature = "dec-ext-flag"))]
pub const DEC_UNDERFLOW: u32 = 0x00000004;

// IEEE 754 groupings for the flags.
// (DEC_Clamped, DEC_Lost_digits, DEC_Rounded, and DEC_Subnormal are not in IEEE 754)

pub const DEC_IEEE_754_DIVISION_BY_ZERO: u32 = DEC_DIVISION_BY_ZERO;
#[cfg(feature = "dec-subset")]
pub const DEC_IEEE_754_INEXACT: u32 = DEC_INEXACT | DEC_LOST_DIGITS;
#[cfg(not(feature = "dec-subset"))]
pub const DEC_IEEE_754_INEXACT: u32 = DEC_INEXACT;

pub const DEC_IEEE_754_INVALID_OPERATION: u32 = DEC_CONVERSION_SYNTAX
  | DEC_DIVISION_IMPOSSIBLE
  | DEC_DIVISION_UNDEFINED
  | DEC_INSUFFICIENT_STORAGE
  | DEC_INVALID_CONTEXT
  | DEC_INVALID_OPERATION;
pub const DEC_IEEE_754_OVERFLOW: u32 = DEC_OVERFLOW;
pub const DEC_IEEE_754_UNDERFLOW: u32 = DEC_UNDERFLOW;

/// Flags which are normally errors (result is qNaN, infinite, or 0).
pub const DEC_ERRORS: u32 = DEC_IEEE_754_DIVISION_BY_ZERO
  | DEC_IEEE_754_INVALID_OPERATION
  | DEC_IEEE_754_OVERFLOW
  | DEC_IEEE_754_UNDERFLOW;

/// Flags which cause a result to become qNaN.
pub const DEC_NANS: u32 = DEC_IEEE_754_INVALID_OPERATION;

/// Flags which are normally for information only (finite results).
#[cfg(feature = "dec-subset")]
pub const DEC_INFORMATION: u32 = DEC_CLAMPED | DEC_ROUNDED | DEC_INEXACT | DEC_LOST_DIGITS;
/// Flags which are normally for information only (finite results).
#[cfg(not(feature = "dec-subset"))]
pub const DEC_INFORMATION: u32 = DEC_CLAMPED | DEC_ROUNDED | DEC_INEXACT;

// IEEE 854 names (for compatibility with older decNumber versions).

pub const DEC_IEEE_854_DIVISION_BY_ZERO: u32 = DEC_IEEE_754_DIVISION_BY_ZERO;
pub const DEC_IEEE_854_INEXACT: u32 = DEC_IEEE_754_INEXACT;
pub const DEC_IEEE_854_INVALID_OPERATION: u32 = DEC_IEEE_754_INVALID_OPERATION;
pub const DEC_IEEE_854_OVERFLOW: u32 = DEC_IEEE_754_OVERFLOW;
pub const DEC_IEEE_854_UNDERFLOW: u32 = DEC_IEEE_754_UNDERFLOW;

// Initialization descriptors, used by dec_context_default function.

/// ANSI X3-274 defaults.
pub const DEC_INIT_BASE: i32 = 0;
/// IEEE 754 defaults, 32-bit.
pub const DEC_INIT_DECIMAL32: i32 = 32;
/// IEEE 754 defaults, 64-bit.
pub const DEC_INIT_DECIMAL64: i32 = 64;
/// IEEE 754 defaults, 128-bit.
pub const DEC_INIT_DECIMAL128: i32 = 128;

// Synonyms.

/// Synonym for [DEC_INIT_DECIMAL32].
pub const DEC_INIT_DEC_SINGLE: i32 = DEC_INIT_DECIMAL32;
/// Synonym for [DEC_INIT_DECIMAL64].
pub const DEC_INIT_DEC_DOUBLE: i32 = DEC_INIT_DECIMAL64;
/// Synonym for [DEC_INIT_DECIMAL128].
pub const DEC_INIT_DEC_QUAD: i32 = DEC_INIT_DECIMAL128;

impl Default for DecContext {
  fn default() -> Self {
    Self {
      digits: 9,                       // 9 digits
      emax: DEC_MAX_EMAX,              // 9-digit exponents
      emin: DEC_MIN_EMIN,              // .. balanced
      round: Rounding::DecRoundHalfUp, // 0.5 rises
      traps: DEC_ERRORS,               // all but informational
      status: 0,                       // cleared
      clamp: 0,                        // no clamping
      #[cfg(feature = "dec-subset")]
      extended: 0, // cleared
    }
  }
}

impl DecContext {
  /// Clear bits in current status.
  ///
  /// `dec_context_clear_status` - clear bits in current status.
  ///
  /// `context` is the context structure to be queried.
  ///
  /// `mask` indicates the bits to be cleared (the status bit that corresponds
  /// to each 1 bit in the mask is cleared).
  ///
  /// Returns updated context.
  ///
  /// No error is possible.
  ///
  pub fn dec_context_clear_status(context: &mut DecContext, mask: u32) -> &DecContext {
    context.status &= !mask;
    context
  }
  /// Initializes a context structure.
  ///
  /// `dec_context_default` - initialize a context structure.
  ///
  /// `context` is the structure to be initialized.
  ///
  /// `kind` selects the required set of default values, one of:
  ///  - DEC_INIT_BASE       - select ANSI X3-274 defaults
  ///  - DEC_INIT_DECIMAL32  - select IEEE 754 defaults, 32-bit
  ///  - DEC_INIT_DECIMAL64  - select IEEE 754 defaults, 64-bit
  ///  - DEC_INIT_DECIMAL128 - select IEEE 754 defaults, 128-bit
  ///
  ///  For any other value a valid context is returned, but with
  ///  Invalid_operation set in the status field.
  ///
  ///  Returns a context structure with the appropriate initial values.
  ///
  pub fn dec_context_default(context: &mut DecContext, kind: i32) -> &DecContext {
    match kind {
      DEC_INIT_BASE => {
        // Use defaults, when the context is initialized, it has already
        // default values set, see implementation of the Default for DecContext.
        // So there is nothing to do in this case.
      }
      DEC_INIT_DECIMAL32 => {
        context.digits = 7; // digits
        context.emax = 96; // Emax
        context.emin = -95; // Emin
        context.round = Rounding::DecRoundHalfEven; // 0.5 to nearest even
        context.traps = 0; // no traps set
        context.clamp = 1; // clamp exponents
        #[cfg(feature = "dec-subset")]
        {
          context.extended = 1; // set
        }
      }
      DEC_INIT_DECIMAL64 => {
        context.digits = 16; // digits
        context.emax = 384; // Emax
        context.emin = -383; // Emin
        context.round = Rounding::DecRoundHalfEven; // 0.5 to nearest even
        context.traps = 0; // no traps set
        context.clamp = 1; // clamp exponents
        #[cfg(feature = "dec-subset")]
        {
          context.extended = 1; // set
        }
      }
      DEC_INIT_DECIMAL128 => {
        context.digits = 34; // digits
        context.emax = 6144; // Emax
        context.emin = -6143; // Emin
        context.round = Rounding::DecRoundHalfEven; // 0.5 to nearest even
        context.traps = 0; // no traps set
        context.clamp = 1; // clamp exponents
        #[cfg(feature = "dec-subset")]
        {
          context.extended = 1; // set
        }
      }
      _ => {
        // invalid kind, use defaults, and trap
        Self::dec_context_set_status(context, DEC_INVALID_OPERATION);
      }
    }
    context
  }
  /// Sets status and raises trap if appropriate.
  ///
  /// `dec_context_set_status` - set status and raise trap if appropriate.
  ///
  /// `context` is the context structure to be updated.
  ///
  /// `status`  is the error code.
  ///
  /// Returns the context structure with updated status.
  ///
  /// Control may never return from this routine, if there is a signal handler and it panics.
  ///
  pub fn dec_context_set_status(context: &mut DecContext, status: u32) -> &DecContext {
    context.status |= status;
    if (status & context.traps) != 0 {
      panic!("SIGFPE");
    }
    context
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rounding_values() {
    assert_eq!(0, Rounding::DecRoundCeiling as u8);
    assert_eq!(1, Rounding::DecRoundUp as u8);
    assert_eq!(2, Rounding::DecRoundHalfUp as u8);
    assert_eq!(3, Rounding::DecRoundHalfEven as u8);
    assert_eq!(4, Rounding::DecRoundHalfDown as u8);
    assert_eq!(5, Rounding::DecRoundDown as u8);
    assert_eq!(6, Rounding::DecRoundFloor as u8);
    assert_eq!(7, Rounding::DecRound05up as u8);
  }

  #[test]
  fn test_default_rounding() {
    assert_eq!(DEC_ROUND_DEFAULT, Rounding::DecRoundHalfEven);
  }

  #[test]
  fn test_dec_context_clear_status() {
    let mut context = DecContext::default();
    assert_eq!(0, context.status);
    context.status = !0;
    assert_eq!(0xFFFFFFFF, context.status);
    DecContext::dec_context_clear_status(&mut context, 0xF);
    assert_eq!(0xFFFFFFF0, context.status);
    DecContext::dec_context_clear_status(&mut context, 0xF0000000);
    assert_eq!(0x0FFFFFF0, context.status);
    DecContext::dec_context_clear_status(&mut context, 0x1000);
    assert_eq!(0x0FFFEFF0, context.status);
  }

  #[test]
  fn test_dec_context_default_base() {
    let mut context = DecContext::default();
    DecContext::dec_context_default(&mut context, DEC_INIT_BASE);
    assert_eq!(9, context.digits);
    assert_eq!(DEC_MAX_EMAX, context.emax);
    assert_eq!(DEC_MIN_EMIN, context.emin);
    assert_eq!(Rounding::DecRoundHalfUp, context.round);
    assert_eq!(DEC_ERRORS, context.traps);
    assert_eq!(0, context.status);
    assert_eq!(0, context.clamp);
    #[cfg(feature = "dec-subset")]
    assert_eq!(0, context.extended);
  }

  #[test]
  fn test_dec_context_default_decimal_32() {
    let mut context = DecContext::default();
    DecContext::dec_context_default(&mut context, DEC_INIT_DECIMAL32);
    assert_eq!(7, context.digits);
    assert_eq!(96, context.emax);
    assert_eq!(-95, context.emin);
    assert_eq!(Rounding::DecRoundHalfEven, context.round);
    assert_eq!(0, context.traps);
    assert_eq!(0, context.status);
    assert_eq!(1, context.clamp);
    #[cfg(feature = "dec-subset")]
    assert_eq!(1, context.extended);
  }

  #[test]
  fn test_dec_context_default_decimal_62() {
    let mut context = DecContext::default();
    DecContext::dec_context_default(&mut context, DEC_INIT_DECIMAL64);
    assert_eq!(16, context.digits);
    assert_eq!(384, context.emax);
    assert_eq!(-383, context.emin);
    assert_eq!(Rounding::DecRoundHalfEven, context.round);
    assert_eq!(0, context.traps);
    assert_eq!(0, context.status);
    assert_eq!(1, context.clamp);
    #[cfg(feature = "dec-subset")]
    assert_eq!(1, context.extended);
  }

  #[test]
  fn test_dec_context_default_decimal_128() {
    let mut context = DecContext::default();
    DecContext::dec_context_default(&mut context, DEC_INIT_DECIMAL128);
    assert_eq!(34, context.digits);
    assert_eq!(6144, context.emax);
    assert_eq!(-6143, context.emin);
    assert_eq!(Rounding::DecRoundHalfEven, context.round);
    assert_eq!(0, context.traps);
    assert_eq!(0, context.status);
    assert_eq!(1, context.clamp);
    #[cfg(feature = "dec-subset")]
    assert_eq!(1, context.extended);
  }

  #[test]
  #[should_panic]
  fn test_dec_context_default_decimal_other() {
    let mut context = DecContext::default();
    DecContext::dec_context_default(&mut context, 37);
  }
}
