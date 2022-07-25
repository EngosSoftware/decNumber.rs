///
pub fn generate_tables() {
  println!();
  print_bin2dpd_table();
  println!();
  print_dpd2bin_table();
}

///
fn print_bin2dpd_table() {
  print!("pub const BIN2DPD: [u16; 1000] = [");
  for i in 0..1000 {
    let dpd = dpd2int(bcd2dpd(int2bcd(i)));
    if i % 19 == 0 {
      print!("\n  ")
    } else if i > 0 && i < 1000 {
      print!(" ");
    }
    print!("0x{:04x},", dpd);
  }
  println!("\n];");
}

///
fn print_dpd2bin_table() {
  print!("pub const DPD2BIN: [u16; 1024] = [");
  for i in 0..1024 {
    let bin = bcd2int(dpd2bcd(int2dpd(i)));
    if i % 19 == 0 {
      print!("\n  ")
    } else if i > 0 && i < 1024 {
      print!(" ");
    }
    print!("0x{:04x},", bin);
  }
  println!("\n];");
}

///
fn bcd2dpd(bcd: [u8; 12]) -> [u8; 10] {
  let (a, b, c, d, e, f, g, h, i, j, k, m) = (
    bcd[0], bcd[1], bcd[2], bcd[3], bcd[4], bcd[5], bcd[6], bcd[7], bcd[8],
    bcd[9], bcd[10], bcd[11],
  );
  [
    b | (a & j) | (a & f & i),
    c | (a & k) | (a & g & i),
    d,
    (f & (!a | !i)) | (!a & e & j) | (e & i),
    g | (!a & e & k) | (a & i),
    h,
    a | e | i,
    a | (e & i) | (!e & j),
    e | (a & i) | (!a & k),
    m,
  ]
}

///
fn dpd2bcd(dpd: [u8; 10]) -> [u8; 12] {
  let (p, q, r, s, t, u, v, w, x, y) = (
    dpd[0], dpd[1], dpd[2], dpd[3], dpd[4], dpd[5], dpd[6], dpd[7], dpd[8],
    dpd[9],
  );
  [
    (v & w) & (!s | t | !x),
    p & (!v | !w | (s & !t & x)),
    q & (!v | !w | (s & !t & x)),
    r,
    v & ((!w & x) | (!t & x) | (s & x)),
    (s & (!v | !x)) | (p & !s & t & v & w & x),
    (t & (!v | !x)) | (q & !s & t & w),
    u,
    v & ((!w & !x) | (w & x & (s | t))),
    (!v & w) | (s & v & !w & x) | (p & w & (!x | (!s & !t))),
    (!v & x) | (t & !w & x) | (q & v & w & (!x | (!s & !t))),
    y,
  ]
}

///
fn int2bcd(mut n: u16) -> [u8; 12] {
  let mut result = [0; 12];
  let mut digit;
  if n < 1000 {
    for i in 0..3 {
      digit = (n % 10) as u8;
      n /= 10;
      result[11 - i * 4] = digit & 0b0001;
      result[10 - i * 4] = (digit & 0b0010) >> 1;
      result[9 - i * 4] = (digit & 0b0100) >> 2;
      result[8 - i * 4] = (digit & 0b1000) >> 3;
    }
    return result;
  }
  [0; 12]
}

///
fn int2dpd(n: u16) -> [u8; 10] {
  let mut result = [0; 10];
  if n < 1024 {
    for i in 0..10 {
      result[9 - i] = ((n >> i) & 0x1) as u8;
    }
    return result;
  }
  [0; 10]
}

///
fn dpd2int(dpd: [u8; 10]) -> u16 {
  let mut result = 0_u16;
  for digit in &dpd {
    result <<= 1;
    result |= (digit & 0x1) as u16;
  }
  result
}

///
fn bcd2int(bcd: [u8; 12]) -> u16 {
  let mut result = 0_u16;
  for i in 0..3 {
    result *= 10;
    let mut digit = 0_u8;
    digit |= bcd[i * 4] << 3;
    digit |= bcd[1 + (i * 4)] << 2;
    digit |= bcd[2 + (i * 4)] << 1;
    digit |= bcd[3 + (i * 4)];
    result += digit as u16;
  }
  result
}

///
pub fn bcd2chr(bcd: [u8; 12]) -> [char; 3] {
  let chars = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
  let mut result = ['0'; 3];
  for i in 0..3 {
    result[i] = chars[(bcd[i * 4] << 3
      | bcd[1 + (i * 4)] << 2
      | bcd[2 + (i * 4)] << 1
      | bcd[3 + (i * 4)]) as usize];
  }
  result
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::dpd::BIN2DPD;

  #[test]
  fn test_generate_bin2dpd_table() {
    let mut work = [0_u16; 1000];
    for i in 0..1000 {
      work[i] = dpd2int(bcd2dpd(int2bcd(i as u16)));
      println!(
        "{:>5} {:>5}  {}",
        i,
        work[i],
        if work[i] == BIN2DPD[i] { "OK" } else { "ERROR" }
      );
    }
  }

  #[test]
  fn test_bcd2chr() {
    assert_eq!(['0', '0', '0'], bcd2chr(int2bcd(0)));
    assert_eq!(['0', '0', '1'], bcd2chr(int2bcd(1)));
    assert_eq!(['0', '1', '0'], bcd2chr(int2bcd(10)));
    assert_eq!(['1', '0', '0'], bcd2chr(int2bcd(100)));
    assert_eq!(['9', '9', '9'], bcd2chr(int2bcd(999)));
  }

  #[test]
  fn test_int2dpd() {
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 0], int2dpd(0));
    assert_eq!([0, 0, 0, 0, 0, 0, 0, 0, 0, 1], int2dpd(1));
    assert_eq!([0, 0, 1, 1, 1, 1, 1, 1, 1, 1], int2dpd(255));
    assert_eq!([1, 1, 1, 1, 1, 1, 1, 1, 1, 1], int2dpd(1023));
  }

  #[test]
  fn test_bcd2int() {
    assert_eq!(0, bcd2int([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(1, bcd2int([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]));
    assert_eq!(80, bcd2int([0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0]));
    assert_eq!(999, bcd2int([1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1]));
  }

  #[test]
  fn test_generate_tables() {
    generate_tables();
  }
}
