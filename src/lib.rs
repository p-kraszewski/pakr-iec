//! Functions to format numbers as decimal (1000 is 1.0k) and IEC (1024 is 1.0ki) up to YOTTA
//! suffix.
//!
//! # Example
//!
//! ```
//! use pakr_iec::*;
//! assert_eq!("1.0", decimal(1));
//! assert_eq!("1.0", iec(1));
//! assert_eq!("1.0k", decimal(1000));
//! assert_eq!("1.0ki", iec(1024));
//! assert_eq!("10.0M", decimal(10_000_000));
//! assert_eq!("10.0Mi", iec(10 * 1024 * 1024));
//! assert_eq!("1.0Y", decimal(1_000_000_000_000_000_000_000_000_u128));
//! assert_eq!("1.0Yi", iec(1_208_925_819_614_629_174_706_176_u128));
//! ```

/// Format value as decimal multipliers (that is in 1000 increments) with one decimal place.
pub fn decimal(val: u128) -> String {
    const MULTS: [&str; 9] = ["", "k", "M", "G", "T", "P", "E", "Z", "Y"];
    let mut s = 0;
    let mut v: u128 = val;
    let mut t: u128 = 0;

    while v >= 1000 && s < MULTS.len() {
        s += 1;
        t = (v % 1000) / 100;
        v /= 1000;
    }

    format!("{}.{}{}", v, t, MULTS[s])
}

/// Format value as IEC multipliers (that is in 1024 increments) with one decimal place.
pub fn iec(val: u128) -> String {
    const MULTS: [&str; 9] = ["", "ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi"];
    let mut s = 0;
    let mut v: u128 = val;
    let mut t: u128 = 0;

    while v >= 1024 && s < MULTS.len() {
        s += 1;

        t = v % 1024;
        v /= 1024;
    }
    t = 10 * t / 1024;

    format!("{}.{}{}", v, t, MULTS[s])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        assert_eq!("1.0", decimal(1));
        assert_eq!("10.0", decimal(10));
        assert_eq!("100.0", decimal(100));
        assert_eq!("1.0k", decimal(1000));
        assert_eq!("1.0k", decimal(1001));
        assert_eq!("1.1k", decimal(1100));
        assert_eq!("10.0k", decimal(10000));
        assert_eq!("10.0M", decimal(10000000));
    }

    #[test]
    fn test_iec() {
        assert_eq!("1.0", iec(1));
        assert_eq!("10.0", iec(10));
        assert_eq!("100.0", iec(100));
        assert_eq!("1000.0", iec(1000));
        assert_eq!("1.0ki", iec(1024));
        assert_eq!("1.0ki", iec(1025));
        assert_eq!("10.0ki", iec(10 * 1024));
        assert_eq!("10.0Mi", iec(10 * 1024 * 1024));
    }

    #[test]
    fn test_iec_boundary() {
        // Exact break would be at 1024+102.4
        assert_eq!("1.0ki", iec(1024 + 102));
        assert_eq!("1.1ki", iec(1024 + 103));

        // Exact break would be at (1024+102.4)*1024, it is at (1024+103)*1024
        assert_eq!("1.0Mi", iec(1154047));
        assert_eq!("1.1Mi", iec(1154048));

        // Break before 2.0
        assert_eq!("1.9Mi", iec(2097151));
        assert_eq!("2.0Mi", iec(2097152));
    }

    #[test]
    fn test_big() {
        assert_eq!("1.0Y", decimal(1_000_000_000_000_000_000_000_000_u128));

        assert_eq!("1.0Yi", iec(1_208_925_819_614_629_174_706_176_u128));
    }
}
