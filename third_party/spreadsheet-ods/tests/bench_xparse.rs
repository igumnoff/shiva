use kparse::{Code, TokenizerError, TokenizerResult};
use nom::combinator::opt;
use nom::{AsBytes, AsChar, Parser};
use std::fmt::{Display, Formatter};
use std::hint::black_box;
use std::str::from_utf8_unchecked;
use std::time::Instant;

#[allow(dead_code)]
// #[test]
fn test_token() {
    let data: [&[u8]; 14] = [
        b"11929".as_bytes(),
        b"66000".as_bytes(),
        b"1".as_bytes(),
        b"11".as_bytes(),
        b"111".as_bytes(),
        b"1111".as_bytes(),
        b"11111".as_bytes(),
        b"-11111".as_bytes(),
        b"-1111".as_bytes(),
        b"-111".as_bytes(),
        b"-11".as_bytes(),
        b"-1".as_bytes(),
        b"AAA".as_bytes(),
        b"-AAA".as_bytes(),
    ];

    let n = 100_000;

    let tt = Instant::now();
    for _ in 0..n {
        for t in data {
            let _ = black_box(token_i16(t.as_bytes()));
        }
    }
    println!("v {:?}", Instant::now().duration_since(tt) / n);

    let tt = Instant::now();
    for _ in 0..n {
        for t in data {
            let _ = black_box(token_i16x(t.as_bytes()));
        }
    }
    println!("vx {:?}", Instant::now().duration_since(tt) / n);

    let tt = Instant::now();
    for _ in 0..n {
        for t in data {
            let _ = black_box(token_i16(t.as_bytes()));
        }
    }
    println!("v {:?}", Instant::now().duration_since(tt) / n);

    let tt = Instant::now();
    for _ in 0..n {
        for t in data {
            let _ = black_box(token_i16x(t.as_bytes()));
        }
    }
    println!("vx {:?}", Instant::now().duration_since(tt) / n);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RCode {
    ANomError,

    Byte,
    Digit,
    Integer,
}

impl Display for RCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Code for RCode {
    const NOM_ERROR: Self = Self::ANomError;
}

// #[cfg(debug_assertions)]
// pub(crate) type KSpan<'s> = TrackSpan<'s, ACode, &'s [u8]>;
// #[cfg(not(debug_assertions))]
pub(crate) type KSpan<'s> = &'s [u8];
// pub(crate) type KParserResult<'s, O> = ParserResult<ACode, KSpan<'s>, O>;
pub(crate) type KTokenizerResult<'s, O> = TokenizerResult<RCode, KSpan<'s>, O>;
// pub(crate) type KParserError<'s> = ParserError<ACode, KSpan<'s>>;
pub(crate) type KTokenizerError<'s> = TokenizerError<RCode, KSpan<'s>>;

pub fn token_i16(input: &[u8]) -> KTokenizerResult<'_, i16> {
    let mut it = input.iter();
    match it.next() {
        Some(b'-') => {}
        Some(v) if v.is_dec_digit() => {}
        _ => return Err(nom::Err::Error(KTokenizerError::new(RCode::Digit, input))),
    }
    for b in it {
        if !b.is_dec_digit() {
            return Err(nom::Err::Error(KTokenizerError::new(RCode::Digit, input)));
        }
    }

    let result = match unsafe { from_utf8_unchecked(input) }.parse::<i16>() {
        Ok(result) => result,
        Err(_) => {
            return Err(nom::Err::Error(KTokenizerError::new(RCode::Integer, input)));
        }
    };

    Ok((input, result))
}

pub fn token_i16x(input: &[u8]) -> KTokenizerResult<'_, i16> {
    let _ = opt(byte(b'-')).and(all_digits).parse(input)?;

    let result = match unsafe { from_utf8_unchecked(input) }.parse::<i16>() {
        Ok(result) => result,
        Err(_) => {
            return Err(nom::Err::Error(KTokenizerError::new(RCode::Integer, input)));
        }
    };

    Ok((input, result))
}

#[inline(always)]
pub fn all_digits(input: &[u8]) -> TokenizerResult<RCode, &[u8], ()> {
    for c in input {
        if !c.is_dec_digit() {
            return Err(nom::Err::Error(KTokenizerError::new(RCode::Digit, input)));
        }
    }
    Ok((&input[input.len()..], ()))
}

#[inline(always)]
pub(crate) fn byte(c: u8) -> impl Fn(&[u8]) -> TokenizerResult<RCode, &[u8], ()> {
    move |i: &[u8]| {
        if i.len() > 0 && i[0] == c {
            Ok((&i[1..], ()))
        } else {
            Err(nom::Err::Error(KTokenizerError::new(RCode::Byte, i)))
        }
    }
}
