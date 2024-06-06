//!
//! Parses different data from a &[u8].
//!
//! For many cases this omits the transformation to a &str

use crate::error::AsStatic;
use crate::sheet::Visibility;
use crate::xlink::{XLinkActuate, XLinkShow, XLinkType};
use crate::OdsError;
use chrono::Duration;
use chrono::NaiveDateTime;
use kparse::prelude::*;
use kparse::{TokenizerError, TokenizerResult};
use nom::character::complete::digit1;
use nom::combinator::{all_consuming, eof, opt};
use nom::number::complete::double;
use nom::sequence::{pair, preceded, terminated, tuple};
use nom::AsChar;
use std::fmt::{Display, Formatter};
use std::str::{from_utf8, from_utf8_unchecked};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub(crate) enum RCode {
    NomError,

    Byte,
    Digit,

    Integer,
    DateTime,
    Bool,
    Duration,
}

impl AsStatic<str> for RCode {
    #[inline]
    fn as_static(&self) -> &'static str {
        match self {
            RCode::NomError => "NomError",
            RCode::Byte => "Byte",
            RCode::Digit => "Digit",
            RCode::Integer => "Integer",
            RCode::DateTime => "DateTime",
            RCode::Bool => "Bool",
            RCode::Duration => "Duration",
        }
    }
}

impl Display for RCode {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Code for RCode {
    const NOM_ERROR: Self = Self::NomError;
}

pub(crate) type KSpan<'s> = &'s [u8];
pub(crate) type KTokenResult<'s, O> = Result<O, nom::Err<TokenizerError<RCode, KSpan<'s>>>>;
pub(crate) type KTokenizerResult<'s, O> = TokenizerResult<RCode, KSpan<'s>, O>;
pub(crate) type KTokenizerError<'s> = TokenizerError<RCode, KSpan<'s>>;

/// Parse as Visibility.
#[inline]
pub(crate) fn parse_visibility(input: KSpan<'_>) -> Result<Visibility, OdsError> {
    match input {
        b"visible" => Ok(Visibility::Visible),
        b"filter" => Ok(Visibility::Filtered),
        b"collapse" => Ok(Visibility::Collapsed),
        _ => Err(OdsError::Ods(format!(
            "Unknown value for table:visibility {}",
            from_utf8(input)?
        ))),
    }
}

/// Parse XLinkActuate enum
#[inline]
pub(crate) fn parse_xlink_actuate(input: KSpan<'_>) -> Result<XLinkActuate, OdsError> {
    match input {
        b"onLoad" => Ok(XLinkActuate::OnLoad),
        b"onRequest" => Ok(XLinkActuate::OnRequest),
        _ => Err(OdsError::Parse(
            "invalid xlink:actuate",
            Some(from_utf8(input)?.to_string()),
        )),
    }
}

/// Parse XLinkShow enum
#[inline]
pub(crate) fn parse_xlink_show(input: KSpan<'_>) -> Result<XLinkShow, OdsError> {
    match input {
        b"new" => Ok(XLinkShow::New),
        b"replace" => Ok(XLinkShow::Replace),
        _ => Err(OdsError::Parse(
            "invalid xlink:show",
            Some(from_utf8(input)?.to_string()),
        )),
    }
}

/// Parse XLinkType enum
#[inline]
pub(crate) fn parse_xlink_type(input: KSpan<'_>) -> Result<XLinkType, OdsError> {
    match input {
        b"simple" => Ok(XLinkType::Simple),
        b"extended" => Ok(XLinkType::Extended),
        b"locator" => Ok(XLinkType::Locator),
        b"arc" => Ok(XLinkType::Arc),
        b"resource" => Ok(XLinkType::Resource),
        b"title" => Ok(XLinkType::Title),
        b"none" => Ok(XLinkType::None),
        _ => Err(OdsError::Parse(
            "invalid xlink:type",
            Some(from_utf8(input)?.to_string()),
        )),
    }
}

/// Parse a attribute value as a currency.
#[inline]
pub(crate) fn parse_string(input: KSpan<'_>) -> Result<String, OdsError> {
    Ok(String::from_utf8_lossy(input).to_string())
}

/// Parse a attribute value as a currency.
#[inline]
pub(crate) fn parse_currency(input: KSpan<'_>) -> Result<String, OdsError> {
    Ok(String::from_utf8_lossy(input).to_string())
}

/// Parse a bool.
#[inline]
pub(crate) fn parse_bool(input: KSpan<'_>) -> Result<bool, OdsError> {
    Ok(token_bool(input)?)
}

/// Parse a u32.
#[inline]
pub(crate) fn parse_u32(input: KSpan<'_>) -> Result<u32, OdsError> {
    Ok(token_u32(input)?)
}

/// Parse a i64.
#[inline]
pub(crate) fn parse_i64(input: KSpan<'_>) -> Result<i64, OdsError> {
    Ok(token_i64(input)?)
}

/// Parse a i32.
#[inline]
pub(crate) fn parse_i32(input: KSpan<'_>) -> Result<i32, OdsError> {
    Ok(token_i32(input)?)
}

/// Parse a i16.
#[inline]
pub(crate) fn parse_i16(input: KSpan<'_>) -> Result<i16, OdsError> {
    Ok(token_i16(input)?)
}

/// Parse a f64.
#[inline]
pub(crate) fn parse_f64(input: KSpan<'_>) -> Result<f64, OdsError> {
    Ok(token_float(input)?)
}

/// Parse a XML Schema datetime.
#[inline]
pub(crate) fn parse_datetime(input: KSpan<'_>) -> Result<NaiveDateTime, OdsError> {
    Ok(token_datetime(input)?)
}

/// Parse a XML Schema time duration.
#[inline]
pub(crate) fn parse_duration(input: KSpan<'_>) -> Result<Duration, OdsError> {
    Ok(token_duration(input)?)
}

#[inline(always)]
fn token_bool(input: KSpan<'_>) -> KTokenResult<'_, bool> {
    match input {
        b"true" => Ok(true),
        b"false" => Ok(false),
        _ => Err(nom::Err::Error(KTokenizerError::new(RCode::Bool, input))),
    }
}

#[inline(always)]
fn token_i16(input: KSpan<'_>) -> KTokenResult<'_, i16> {
    sign_all_digits(input)?;

    let result = match unsafe { from_utf8_unchecked(input) }.parse::<i16>() {
        Ok(result) => result,
        Err(_) => {
            return Err(nom::Err::Error(KTokenizerError::new(RCode::Integer, input)));
        }
    };

    Ok(result)
}

#[inline(always)]
fn token_u32(input: KSpan<'_>) -> KTokenResult<'_, u32> {
    all_digits(input)?;

    let result = match unsafe { from_utf8_unchecked(input) }.parse::<u32>() {
        Ok(result) => result,
        Err(_) => {
            return Err(nom::Err::Error(KTokenizerError::new(RCode::Integer, input)));
        }
    };

    Ok(result)
}

#[inline(always)]
fn token_i32(input: KSpan<'_>) -> KTokenResult<'_, i32> {
    sign_all_digits(input)?;

    let result = match unsafe { from_utf8_unchecked(input) }.parse::<i32>() {
        Ok(result) => result,
        Err(_) => {
            return Err(nom::Err::Error(KTokenizerError::new(RCode::Integer, input)));
        }
    };

    Ok(result)
}

#[inline(always)]
fn token_i64(input: KSpan<'_>) -> KTokenResult<'_, i64> {
    sign_all_digits(input)?;

    let result = match unsafe { from_utf8_unchecked(input) }.parse::<i64>() {
        Ok(result) => result,
        Err(_) => {
            return Err(nom::Err::Error(KTokenizerError::new(RCode::Integer, input)));
        }
    };

    Ok(result)
}

#[inline(always)]
fn token_float(input: KSpan<'_>) -> KTokenResult<'_, f64> {
    let (_, result) = all_consuming(double)(input)?;
    Ok(result)
}

// Part of a date/duration. An unsigned integer, but for chrono we need an i64.
#[inline]
fn token_datepart(input: KSpan<'_>) -> KTokenizerResult<'_, i64> {
    let (rest, result) = digit1(input)?;

    let result = match unsafe { from_utf8_unchecked(result) }.parse::<i64>() {
        Ok(result) => result,
        Err(_) => {
            return Err(nom::Err::Error(KTokenizerError::new(RCode::Integer, input)));
        }
    };

    Ok((rest, result))
}

// Part of a date/duration. Parses an integer as nanoseconds but with
// the caveat that there can be arbitrary many trailing 0s omitted.
#[inline]
fn token_nano(input: KSpan<'_>) -> KTokenizerResult<'_, i64> {
    let (rest, result) = digit1(input)?;

    let mut v = 0i64;
    for i in 0..9 {
        if i < result.len() {
            v *= 10;
            v += (result[i] - b'0') as i64;
        } else {
            v *= 10;
        }
    }
    Ok((rest, v))
}

#[inline(always)]
fn token_datetime(input: KSpan<'_>) -> KTokenResult<'_, NaiveDateTime> {
    let (_, (minus, year, _, month, _, day, time, _)) = terminated(
        tuple((
            opt(byte(b'-')),
            token_datepart,
            byte(b'-'),
            token_datepart,
            byte(b'-'),
            token_datepart,
            opt(tuple((
                byte(b'T'),
                token_datepart,
                byte(b':'),
                token_datepart,
                byte(b':'),
                token_datepart,
                opt(tuple((byte(b'.'), token_nano))),
            ))),
            opt(byte(b'Z')),
        )),
        eof,
    )(input)?;

    let sign = match minus {
        Some(_) => -1,
        None => 1,
    };

    let mut p = chrono::format::Parsed::new();
    p.year = Some((sign * year) as i32);
    p.month = Some(month as u32);
    p.day = Some(day as u32);
    if let Some((_, hour, _, minute, _, second, nanos)) = time {
        p.hour_div_12 = Some((hour / 12) as u32);
        p.hour_mod_12 = Some((hour % 12) as u32);
        p.minute = Some(minute as u32);
        p.second = Some(second as u32);
        if let Some((_, nanos)) = nanos {
            p.nanosecond = Some(nanos as u32);
        }
    } else {
        p.hour_div_12 = Some(0);
        p.hour_mod_12 = Some(0);
        p.minute = Some(0);
        p.second = Some(0);
    }
    match p.to_naive_datetime_with_offset(0) {
        Ok(v) => Ok(v),
        Err(_) => Err(nom::Err::Error(KTokenizerError::new(
            RCode::DateTime,
            input,
        ))),
    }
}

#[inline(always)]
fn token_duration(input: KSpan<'_>) -> KTokenResult<'_, Duration> {
    let (_, (_, day, time)) = all_consuming(tuple((
        byte(b'P'),
        // these do not occur?
        //opt(terminated(token_datepart, byte(b'Y'))),
        //opt(terminated(token_datepart, byte(b'M'))),
        opt(terminated(token_datepart, byte(b'D'))),
        opt(tuple((
            byte(b'T'),
            opt(terminated(token_datepart, byte(b'H'))),
            opt(terminated(token_datepart, byte(b'M'))),
            terminated(
                pair(token_datepart, opt(preceded(byte(b'.'), token_nano))),
                byte(b'S'),
            ),
        ))),
    )))(input)?;

    let mut result = Duration::try_seconds(0).ok_or(nom::Err::Error(KTokenizerError::new(
        RCode::Duration,
        input,
    )))?;
    if let Some(day) = day {
        result += Duration::try_days(day).ok_or(nom::Err::Error(KTokenizerError::new(
            RCode::Duration,
            input,
        )))?;
    }
    if let Some((_, hour, minute, (second, nanos))) = time {
        if let Some(hour) = hour {
            result += Duration::try_hours(hour).ok_or(nom::Err::Error(KTokenizerError::new(
                RCode::Duration,
                input,
            )))?;
        }
        if let Some(minute) = minute {
            result += Duration::try_minutes(minute).ok_or(nom::Err::Error(
                KTokenizerError::new(RCode::Duration, input),
            ))?;
        }
        result += Duration::try_seconds(second).ok_or(nom::Err::Error(KTokenizerError::new(
            RCode::Duration,
            input,
        )))?;
        if let Some(nanos) = nanos {
            result += Duration::nanoseconds(nanos);
        }
    }

    Ok(result)
}

#[inline(always)]
fn all_digits(input: KSpan<'_>) -> KTokenResult<'_, ()> {
    for c in input {
        if !c.is_dec_digit() {
            return Err(nom::Err::Error(KTokenizerError::new(RCode::Digit, input)));
        }
    }
    Ok(())
}

#[inline(always)]
fn sign_all_digits(input: KSpan<'_>) -> KTokenResult<'_, ()> {
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
    Ok(())
}

#[inline(always)]
pub(crate) fn byte(c: u8) -> impl Fn(KSpan<'_>) -> KTokenizerResult<'_, ()> {
    move |i: KSpan<'_>| {
        if !i.is_empty() && i[0] == c {
            Ok((&i[1..], ()))
        } else {
            Err(nom::Err::Error(KTokenizerError::new(RCode::Byte, i)))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::io::parse::{
        parse_bool, parse_datetime, parse_duration, parse_f64, parse_i32, parse_u32, token_nano,
    };
    use crate::OdsError;

    #[test]
    fn test_u32() -> Result<(), OdsError> {
        assert_eq!(parse_u32(b"1234")?, 1234);
        parse_u32(b"123456789000").unwrap_err();
        parse_u32(b"1234 ").unwrap_err();
        parse_u32(b"-1234 ").unwrap_err();
        parse_u32(b"-1234").unwrap_err();

        Ok(())
    }

    #[test]
    fn test_i32() -> Result<(), OdsError> {
        assert_eq!(parse_i32(b"1234")?, 1234);
        assert_eq!(parse_i32(b"-1234")?, -1234);
        parse_i32(b"1234 ").unwrap_err();
        parse_i32(b"-1234 ").unwrap_err();
        parse_i32(b"123456789000").unwrap_err();

        Ok(())
    }

    #[test]
    fn test_float() -> Result<(), OdsError> {
        assert_eq!(parse_f64(b"1234")?, 1234.);
        assert_eq!(parse_f64(b"-1234")?, -1234.);
        assert_eq!(parse_f64(b"123456789000")?, 123456789000.);
        assert_eq!(parse_f64(b"1234.5678")?, 1234.5678);
        parse_f64(b"1234 ").unwrap_err();
        parse_f64(b"-1234 ").unwrap_err();

        Ok(())
    }

    #[test]
    fn test_datetime() -> Result<(), OdsError> {
        assert_eq!(parse_datetime(b"19999-01-01")?.timestamp(), 568940284800);
        assert_eq!(parse_datetime(b"1999-01-01")?.timestamp(), 915148800);
        assert_eq!(parse_datetime(b"-45-01-01")?.timestamp(), -63587289600);
        assert_eq!(parse_datetime(b"2004-02-29")?.timestamp(), 1078012800);
        assert_eq!(parse_datetime(b"2000-02-29")?.timestamp(), 951782400);

        assert_eq!(
            parse_datetime(b"2000-01-01T11:22:33")?.timestamp(),
            946725753
        );
        assert_eq!(
            parse_datetime(b"2000-01-01T11:22:33.1234")?.timestamp(),
            946725753
        );
        assert_eq!(
            parse_datetime(b"2000-01-01T11:22:33.123456789111")?.timestamp(),
            946725753
        );

        Ok(())
    }

    #[test]
    fn test_duration() -> Result<(), OdsError> {
        assert_eq!(parse_duration(b"PT12H12M12S")?.num_milliseconds(), 43932000);
        assert_eq!(
            parse_duration(b"PT12H12M12.223S")?.num_milliseconds(),
            43932223
        );
        Ok(())
    }

    #[test]
    fn test_bool() -> Result<(), OdsError> {
        assert_eq!(parse_bool(b"true")?, true);
        assert_eq!(parse_bool(b"false")?, false);
        parse_bool(b"ffoso").unwrap_err();
        Ok(())
    }

    #[test]
    fn test_nano() -> Result<(), OdsError> {
        assert_eq!(token_nano(b"123")?.1, 123000000i64);
        assert_eq!(token_nano(b"123456789")?.1, 123456789i64);
        assert_eq!(token_nano(b"1234567897777")?.1, 123456789i64);
        Ok(())
    }
}
