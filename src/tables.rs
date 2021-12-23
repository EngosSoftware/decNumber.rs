///
pub fn generate_tables() {
  print_bin2dpd_table()
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
fn bcd2dpd(bcd: [u8; 12]) -> [u8; 10] {
  let (a, b, c, d, e, f, g, h, i, j, k, m) = (bcd[0], bcd[1], bcd[2], bcd[3], bcd[4], bcd[5], bcd[6], bcd[7], bcd[8], bcd[9], bcd[10], bcd[11]);
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
fn dpd2int(dpd: [u8; 10]) -> u16 {
  let mut result = 0_u16;
  for digit in &dpd {
    result <<= 1;
    result |= (digit & 0x1) as u16;
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
      println!("{:>5} {:>5}  {}", i, work[i], if work[i] == BIN2DPD[i] { "OK" } else { "ERROR" });
    }
  }

  #[test]
  fn test_generate_tables() {
    generate_tables();
  }
}
