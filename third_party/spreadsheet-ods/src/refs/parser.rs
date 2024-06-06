use crate::error::AsStatic;
use crate::refs::parser::parser::{parse_col, parse_iri, parse_row, parse_sheet_name};
use crate::refs::parser::tokens::colon;
use crate::{CellRange, CellRef, ColRange, RowRange};
use kparse::prelude::*;
use kparse::{TokenizerError, TokenizerResult};
use nom::character::complete::multispace1;
use nom::combinator::all_consuming;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use std::fmt::{Display, Formatter};
use CRCode::*;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CRCode {
    CRNomError,

    CRCellRangeList,
    CRCellRange,
    CRColRange,
    CRRowRange,
    CRCellRef,

    CRIri,

    CRCol,
    CRColInteger,
    CRColon,
    CRDollar,
    CRDot,
    CRHash,
    CRRow,
    CRRowInteger,
    CRSingleQuoteEnd,
    CRSingleQuoteStart,
    CRString,
    CRUnquotedName,
}

impl AsStatic<str> for CRCode {
    fn as_static(&self) -> &'static str {
        match self {
            CRNomError => "NomError",
            CRCellRangeList => "CellRangeList",
            CRCellRange => "CellRange",
            CRColRange => "ColRange",
            CRRowRange => "RowRange",
            CRCellRef => "CellRef",
            CRIri => "Iri",
            CRCol => "Col",
            CRColInteger => "ColInteger",
            CRColon => "Colon",
            CRDollar => "Dollar",
            CRDot => "Dot",
            CRHash => "Hash",
            CRRow => "Row",
            CRRowInteger => "RowInteger",
            CRSingleQuoteEnd => "SingleQuoteEnd",
            CRSingleQuoteStart => "SingleQuoteStart",
            CRString => "String",
            CRUnquotedName => "UnquotedName",
        }
    }
}

impl Display for CRCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CRNomError => "Nom",
            CRCellRangeList => "cell-range list",
            CRCellRange => "cell-range",
            CRColRange => "col-range",
            CRRowRange => "row-range",
            CRCellRef => "cell-ref",
            CRIri => "iri",
            CRCol => "col",
            CRColInteger => "col int",
            CRColon => ":",
            CRDollar => "$",
            CRDot => ".",
            CRHash => "#",
            CRRow => "row",
            CRRowInteger => "row int",
            CRSingleQuoteEnd => "' start",
            CRSingleQuoteStart => "' end",
            CRString => "str",
            CRUnquotedName => "unquoted",
        };
        write!(f, "{}", str)
    }
}

impl Code for CRCode {
    const NOM_ERROR: Self = Self::CRNomError;
}

define_span!(pub(crate) KSpan = CRCode, str);
pub(crate) type KTokenizerResult<'s, O> = TokenizerResult<CRCode, KSpan<'s>, O>;
pub(crate) type KTokenizerError<'s> = TokenizerError<CRCode, KSpan<'s>>;

pub(crate) fn parse_cell_ref(input: KSpan<'_>) -> KTokenizerResult<'_, CellRef> {
    Track.enter(CRCellRef, input);

    let (rest, (iri, table, (abs_col, col), (abs_row, row))) = all_consuming(tuple((
        parse_iri, //
        parse_sheet_name,
        parse_col,
        parse_row,
    )))(input)
    .track()?;

    Track.ok(
        rest,
        input,
        CellRef::new_all(iri, table, abs_row, row, abs_col, col),
    )
}

pub(crate) fn parse_cell_range_list(
    input: KSpan<'_>,
) -> KTokenizerResult<'_, Option<Vec<CellRange>>> {
    Track.enter(CRCellRangeList, input);

    let (rest, vec) = separated_list0(multispace1, parse_cell_range)(input).track()?;

    if vec.is_empty() {
        Track.ok(rest, input, None)
    } else {
        Track.ok(rest, input, Some(vec))
    }
}

pub(crate) fn parse_cell_range(input: KSpan<'_>) -> KTokenizerResult<'_, CellRange> {
    Track.enter(CRCellRange, input);

    let (
        rest,
        (
            iri,
            table,
            (abs_col, col),
            (abs_row, row),
            _,
            to_table,
            (abs_to_col, to_col),
            (abs_to_row, to_row),
        ),
    ) = tuple((
        parse_iri,
        parse_sheet_name,
        parse_col,
        parse_row,
        colon,
        parse_sheet_name,
        parse_col,
        parse_row,
    ))(input)
    .track()?;

    Track.ok(
        rest,
        input,
        CellRange::new_all(
            iri, table, abs_row, row, abs_col, col, to_table, abs_to_row, to_row, abs_to_col,
            to_col,
        ),
    )
}

pub(crate) fn parse_col_range(input: KSpan<'_>) -> KTokenizerResult<'_, ColRange> {
    Track.enter(CRColRange, input);

    let (rest, (iri, table, (abs_col, col), _, to_table, (abs_to_col, to_col))) = tuple((
        parse_iri,
        parse_sheet_name,
        parse_col,
        colon,
        parse_sheet_name,
        parse_col,
    ))(input)
    .track()?;

    Track.ok(
        rest,
        input,
        ColRange::new_all(iri, table, abs_col, col, to_table, abs_to_col, to_col),
    )
}

pub(crate) fn parse_row_range(input: KSpan<'_>) -> KTokenizerResult<'_, RowRange> {
    Track.enter(CRRowRange, input);

    let (rest, (iri, table, (abs_row, row), _, to_table, (abs_to_row, to_row))) = tuple((
        parse_iri,
        parse_sheet_name,
        parse_row,
        colon,
        parse_sheet_name,
        parse_row,
    ))(input)
    .track()?;

    Track.ok(
        rest,
        input,
        RowRange::new_all(iri, table, abs_row, row, to_table, abs_to_row, to_row),
    )
}

mod conv {
    use crate::refs::parser::KSpan;
    #[cfg(not(debug_assertions))]
    use kparse::prelude::*;
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use std::num::IntErrorKind;
    use std::str::FromStr;

    /// Replaces two single quotes (') with a single on.
    /// Strips one leading and one trailing quote.
    pub(crate) fn unquote_single(i: KSpan<'_>) -> String {
        let i = match i.strip_prefix('\'') {
            None => i.fragment(),
            Some(s) => s,
        };
        let i = match i.strip_suffix('\'') {
            None => i,
            Some(s) => s,
        };

        i.replace("''", "'")
    }

    /// Parse a bool if a '$' exists.
    pub(crate) fn try_bool_from_abs_flag(i: Option<KSpan<'_>>) -> bool {
        if let Some(i) = i {
            *i.fragment() == "$"
        } else {
            false
        }
    }

    /// Error for try_u32_from_rowname.
    #[allow(variant_size_differences)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) enum ParseRownameError {
        /// Value being parsed is empty.
        ///
        /// This variant will be constructed when parsing an empty string.
        Empty,
        /// Contains an invalid digit in its Context.
        ///
        /// Among other causes, this variant will be constructed when parsing a string that
        /// contains a non-ASCII char.
        ///
        /// This variant is also constructed when a `+` or `-` is misplaced within a string
        /// either on its own or in the middle of a number.
        InvalidDigit,
        /// Integer is too large to store in target integer type.
        PosOverflow,
        /// Integer is too small to store in target integer type.
        NegOverflow,
        /// Value was Zero
        ///
        /// This variant will be emitted when the parsing string has a value of zero, which
        /// would be illegal for non-zero types.
        Zero,
        /// Something else.
        Other,
    }

    impl Display for ParseRownameError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                ParseRownameError::Empty => write!(f, "Input was empty")?,
                ParseRownameError::InvalidDigit => write!(f, "Invalid digit")?,
                ParseRownameError::PosOverflow => write!(f, "Positive overflow")?,
                ParseRownameError::NegOverflow => write!(f, "Negative overflow")?,
                ParseRownameError::Zero => write!(f, "Zero")?,
                ParseRownameError::Other => write!(f, "Other")?,
            }
            Ok(())
        }
    }

    impl Error for ParseRownameError {}

    /// Parse a row number to a row index.
    #[allow(clippy::explicit_auto_deref)]
    pub(crate) fn try_u32_from_rowname(i: KSpan<'_>) -> Result<u32, ParseRownameError> {
        match u32::from_str(i.fragment()) {
            Ok(v) if v > 0 => Ok(v - 1),
            Ok(_v) => Err(ParseRownameError::Zero),
            Err(e) => Err(match e.kind() {
                IntErrorKind::Empty => ParseRownameError::Empty,
                IntErrorKind::InvalidDigit => ParseRownameError::InvalidDigit,
                IntErrorKind::PosOverflow => ParseRownameError::PosOverflow,
                IntErrorKind::NegOverflow => ParseRownameError::NegOverflow,
                IntErrorKind::Zero => ParseRownameError::Zero,
                _ => ParseRownameError::Other,
            }),
        }
    }

    /// Error for try_u32_from_colname.
    #[allow(variant_size_differences)]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) enum ParseColnameError {
        /// Invalid column character.
        InvalidChar,
        /// Invalid column name.
        InvalidColname,
    }

    impl Display for ParseColnameError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            match self {
                ParseColnameError::InvalidChar => {
                    write!(f, "Invalid char")?;
                }
                ParseColnameError::InvalidColname => {
                    write!(f, "Invalid colname")?;
                }
            }
            Ok(())
        }
    }

    impl Error for ParseColnameError {}

    /// Parse a col label to a column index.
    pub(crate) fn try_u32_from_colname(i: KSpan<'_>) -> Result<u32, ParseColnameError> {
        let mut col = 0u32;

        for c in (*i).chars() {
            if !c.is_ascii_uppercase() {
                return Err(ParseColnameError::InvalidChar);
            }

            let mut v = c as u32 - b'A' as u32;
            if v == 25 {
                v = 0;
                col = (col + 1) * 26;
            } else {
                v += 1;
                col *= 26;
            }
            col += v;
        }

        if col == 0 {
            Err(ParseColnameError::InvalidColname)
        } else {
            Ok(col - 1)
        }
    }
}

#[allow(clippy::module_inception)]
mod parser {
    use crate::refs::parser::tokens::{
        col, dollar_nom, dot, hashtag, row, single_quoted_string, unquoted_sheet_name,
    };
    use crate::refs::parser::CRCode::*;
    use crate::refs::parser::{conv, KSpan, KTokenizerError, KTokenizerResult};
    use kparse::combinators::track;
    use kparse::prelude::*;
    use nom::combinator::opt;
    use nom::sequence::{terminated, tuple};
    use nom::Parser;

    pub(crate) fn parse_iri(input: KSpan<'_>) -> KTokenizerResult<'_, Option<String>> {
        let parsed = track(
            CRIri, //
            terminated(single_quoted_string, hashtag),
        )
        .parse(input);

        let (rest, iri) = match parsed {
            Ok((rest, iri)) => (rest, Some(iri)),
            Err(nom::Err::Error(e)) if e.code == CRSingleQuoteStart => (input, None),
            Err(nom::Err::Error(e)) if e.code == CRHash => (input, None),
            Err(e) => return Err(e),
        };

        Ok((rest, iri))
    }
    /// Sheet name
    pub(crate) fn parse_sheet_name(input: KSpan<'_>) -> KTokenizerResult<'_, Option<String>> {
        tuple((
            opt(dollar_nom),
            opt(single_quoted_string.or(unquoted_sheet_name)),
            dot,
        ))
        .map(|(_, sheet_name, _)| sheet_name)
        .parse(input)
    }

    pub(crate) fn parse_row(input: KSpan<'_>) -> KTokenizerResult<'_, (bool, u32)> {
        track(CRRow, row)
            .map_res(|(abs, row)| {
                let abs = conv::try_bool_from_abs_flag(abs);
                let row = match conv::try_u32_from_rowname(row) {
                    Ok(v) => v,
                    Err(_) => return Err(KTokenizerError::new(CRRowInteger, row).error()),
                };
                Ok((abs, row))
            })
            .parse(input)
    }

    pub(crate) fn parse_col(input: KSpan<'_>) -> KTokenizerResult<'_, (bool, u32)> {
        track(CRCol, col)
            .map_res(|(abs, col)| {
                let abs = conv::try_bool_from_abs_flag(abs);
                let col = match conv::try_u32_from_colname(col) {
                    Ok(v) => v,
                    Err(_) => return Err(KTokenizerError::new(CRColInteger, col).error()),
                };
                Ok((abs, col))
            })
            .parse(input)
    }
}

mod tokens {
    use crate::refs::parser::conv::unquote_single;
    use crate::refs::parser::CRCode::*;
    use crate::refs::parser::{KSpan, KTokenizerResult};
    use kparse::combinators::pchar;
    use kparse::prelude::*;
    use nom::branch::alt;
    use nom::bytes::complete::{tag, take_while1};
    use nom::character::complete::{alpha1, char as nchar, digit1};
    use nom::combinator::{opt, recognize};
    use nom::multi::{count, many0};
    use nom::sequence::tuple;
    use nom::Parser;

    const SINGLE_QUOTE: char = '\'';

    /// SingleQuoted ::= "'" ([^'] | "''")+ "'"
    /// Parse a quoted string. A double quote within is an escaped quote.
    /// Returns the string within the outer quotes. The double quotes are not
    /// reduced.
    pub(crate) fn single_quoted_string(input: KSpan<'_>) -> KTokenizerResult<'_, String> {
        recognize(tuple((
            pchar(SINGLE_QUOTE).with_code(CRSingleQuoteStart),
            recognize(many0(alt((
                take_while1(|v| v != SINGLE_QUOTE),
                recognize(count(nchar(SINGLE_QUOTE), 2)),
            ))))
            .with_code(CRString),
            pchar(SINGLE_QUOTE).with_code(CRSingleQuoteEnd),
        )))
        .map(unquote_single)
        .parse(input)
    }

    /// Hashtag
    pub(crate) fn hashtag(input: KSpan<'_>) -> KTokenizerResult<'_, KSpan<'_>> {
        tag("#").with_code(CRHash).parse(input)
    }

    const T: bool = true;
    const F: bool = false;

    const SHEET_NAME: [bool; 128] = [
        F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, //
        F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, //
        F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, //  !"#$%&'()*+,-./
        T, T, T, T, T, T, T, T, T, T, F, F, F, F, F, F, // 0123456789:;<=>?
        F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, // @ABCDEFGHIJKLMNO
        T, T, T, T, T, T, T, T, T, T, T, F, F, F, F, T, // PQRSTUVWXYZ[\]^_
        F, T, T, T, T, T, T, T, T, T, T, T, T, T, T, T, // `abcdefghijklmno
        T, T, T, T, T, T, T, T, T, T, T, F, F, F, F, F, // pqrstuvwxyz{|}~
    ];

    // SheetName ::= QuotedSheetName | '$'? [^\]\. #$']+
    // QuotedSheetName ::= '$'? SingleQuoted
    pub(crate) fn unquoted_sheet_name(i: KSpan<'_>) -> KTokenizerResult<'_, String> {
        take_while1(|v| {
            if (v as i32) < 128 {
                SHEET_NAME[v as usize]
            } else {
                true
            }
        })
        .with_code(CRUnquotedName)
        .map(|v: KSpan<'_>| v.fragment().to_string())
        .parse(i)
    }

    /// Parse dollar
    pub(crate) fn dollar_nom(input: KSpan<'_>) -> KTokenizerResult<'_, KSpan<'_>> {
        tag("$").with_code(CRDollar).parse(input)
    }

    /// Parse dot
    pub(crate) fn dot(input: KSpan<'_>) -> KTokenizerResult<'_, KSpan<'_>> {
        tag(".").with_code(CRDot).parse(input)
    }

    /// Parse colon
    pub(crate) fn colon(input: KSpan<'_>) -> KTokenizerResult<'_, KSpan<'_>> {
        tag(":").with_code(CRColon).parse(input)
    }

    // Column ::= '$'? [A-Z]+
    /// Column label
    pub(crate) fn col(i: KSpan<'_>) -> KTokenizerResult<'_, (Option<KSpan<'_>>, KSpan<'_>)> {
        tuple((
            opt(dollar_nom), //
            alpha1.with_code(CRCol),
        ))
        .parse(i)
    }

    // Row ::= '$'? [1-9] [0-9]*
    /// Row label
    pub(crate) fn row(i: KSpan<'_>) -> KTokenizerResult<'_, (Option<KSpan<'_>>, KSpan<'_>)> {
        tuple((
            opt(dollar_nom), //
            digit1.with_code(CRRow),
        ))
        .parse(i)
    }
}

#[cfg(test)]
mod tests {
    use crate::refs::parser::tokens::{col, row};
    use crate::refs::parser::CRCode::*;
    use crate::refs::parser::{parse_cell_range, parse_cell_ref, parse_col_range, parse_row_range};
    use crate::{CellRange, CellRef, ColRange, RowRange};
    use kparse::test::{str_parse, CheckTrace};

    const R: CheckTrace = CheckTrace;

    #[test]
    pub(crate) fn test_col() {
        str_parse(&mut None, "", col).err(CRCol).q(R);
        str_parse(&mut None, "$A", col).ok_any().q(R);
        str_parse(&mut None, "$", col).err(CRCol).q(R);
        str_parse(&mut None, "A", col).ok_any().q(R);
        str_parse(&mut None, "$A ", col).ok_any().rest(" ").q(R);
    }

    #[test]
    pub(crate) fn test_row() {
        str_parse(&mut None, "", row).err(CRRow).q(R);
        str_parse(&mut None, "$1", row).ok_any().q(R);
        str_parse(&mut None, "$", row).err(CRRow).q(R);
        str_parse(&mut None, "1", row).ok_any().q(R);
        str_parse(&mut None, "$1 ", row).ok_any().rest(" ").q(R);
    }

    #[test]
    pub(crate) fn test_cellref() {
        fn iri(result: &CellRef, test: &str) -> bool {
            match result.iri() {
                Some(iri) => iri == test,
                None => false,
            }
        }
        fn table(result: &CellRef, test: &str) -> bool {
            match result.table() {
                Some(table) => table == test,
                None => false,
            }
        }
        fn row_col(result: &CellRef, test: &(u32, u32)) -> bool {
            (result.row(), result.col()) == *test
        }
        fn absolute(result: &CellRef, test: &(bool, bool)) -> bool {
            (result.row_abs(), result.col_abs()) == *test
        }

        str_parse(&mut None, "", parse_cell_ref).err_any().q(R);
        str_parse(&mut None, "'iri'#.A1", parse_cell_ref)
            .ok(iri, "iri")
            .q(R);
        str_parse(&mut None, "'iri'#.A1", parse_cell_ref)
            .ok(iri, "iri")
            .q(R);
        str_parse(&mut None, "'sheet'.A1", parse_cell_ref)
            .ok(table, "sheet")
            .q(R);
        str_parse(&mut None, ".A1", parse_cell_ref)
            .ok(row_col, &(0, 0))
            .ok(absolute, &(false, false))
            .q(R);
        str_parse(&mut None, ".A", parse_cell_ref).err(CRRow).q(R);
        str_parse(&mut None, ".1", parse_cell_ref).err(CRCol).q(R);
        str_parse(&mut None, "A1", parse_cell_ref).err(CRDot).q(R);
        str_parse(&mut None, ".$A$1", parse_cell_ref)
            .ok(row_col, &(0, 0))
            .ok(absolute, &(true, true))
            .q(R);
        str_parse(&mut None, ".$A $1", parse_cell_ref)
            .err(CRRow)
            .q(R);
        str_parse(&mut None, ".$ A$1", parse_cell_ref)
            .err(CRCol)
            .q(R);
        str_parse(&mut None, ".$A$ 1", parse_cell_ref)
            .err(CRRow)
            .q(R);
        str_parse(&mut None, ".$A$$1", parse_cell_ref)
            .err(CRRow)
            .q(R);
        str_parse(&mut None, ".$$A$$1", parse_cell_ref)
            .err(CRCol)
            .q(R);
        str_parse(&mut None, "'iri'#$'sheet'.$A$1", parse_cell_ref)
            .ok(iri, "iri")
            .ok(table, "sheet")
            .q(R);
    }

    #[test]
    pub(crate) fn test_cellrange() {
        fn iri(result: &CellRange, test: &str) -> bool {
            match result.iri() {
                Some(iri) => iri == test,
                None => false,
            }
        }
        fn table(result: &CellRange, test: &str) -> bool {
            match result.table() {
                Some(table) => table == test,
                None => false,
            }
        }
        fn to_table(result: &CellRange, test: &str) -> bool {
            match result.to_table() {
                Some(to_table) => to_table == test,
                None => false,
            }
        }
        fn row_col(result: &CellRange, test: &(u32, u32)) -> bool {
            (result.row(), result.col()) == *test
        }
        fn to_row_col(result: &CellRange, test: &(u32, u32)) -> bool {
            (result.to_row(), result.to_col()) == *test
        }
        fn absolute(result: &CellRange, test: &(bool, bool)) -> bool {
            (result.row_abs(), result.col_abs()) == *test
        }
        fn to_absolute(result: &CellRange, test: &(bool, bool)) -> bool {
            (result.to_row_abs(), result.to_col_abs()) == *test
        }

        str_parse(&mut None, "", parse_cell_range).err(CRDot).q(R);
        str_parse(&mut None, "'iri'#.A1:.C3", parse_cell_range)
            .ok(iri, "iri")
            .q(R);
        str_parse(&mut None, "'sheet'.A1:.C3", parse_cell_range)
            .ok(table, "sheet")
            .q(R);
        str_parse(&mut None, ".A1:.C3", parse_cell_range)
            .ok(row_col, &(0, 0))
            .ok(to_row_col, &(2, 2))
            .q(R);
        str_parse(&mut None, ".$A$1:.$C$3", parse_cell_range)
            .ok(row_col, &(0, 0))
            .ok(absolute, &(true, true))
            .ok(to_row_col, &(2, 2))
            .ok(to_absolute, &(true, true))
            .q(R);
        str_parse(&mut None, "'fun'.$A$1:'nofun'.$C$3", parse_cell_range)
            .ok(table, "fun")
            .ok(to_table, "nofun")
            .q(R);
        str_parse(&mut None, ".A1:.C3", parse_cell_range)
            .ok_any()
            .q(R);
        str_parse(&mut None, ".A1:.3", parse_cell_range)
            .err(CRCol)
            .q(R);
        str_parse(&mut None, ".A1:.C", parse_cell_range)
            .err(CRRow)
            .q(R);
        str_parse(&mut None, ".A:.C3", parse_cell_range)
            .err(CRRow)
            .q(R);
        str_parse(&mut None, ".1:.C3", parse_cell_range)
            .err(CRCol)
            .q(R);
        str_parse(&mut None, ":.C3", parse_cell_range)
            .err(CRDot)
            .q(R);
        str_parse(&mut None, "A1:C3", parse_cell_range)
            .err(CRDot)
            .q(R);
        str_parse(
            &mut None,
            "'external'#'fun'.$A$1:'nofun'.$C$3",
            parse_cell_range,
        )
        .ok(table, "fun")
        .ok(to_table, "nofun")
        .q(R);
    }

    #[test]
    pub(crate) fn colrange() {
        fn iri(result: &ColRange, test: &str) -> bool {
            match result.iri() {
                Some(iri) => iri == test,
                None => false,
            }
        }
        fn sheet_name(result: &ColRange, test: &str) -> bool {
            match result.table() {
                Some(table) => table == test,
                None => false,
            }
        }
        fn col_col(result: &ColRange, test: &(u32, u32)) -> bool {
            (result.col(), result.to_col()) == *test
        }

        str_parse(&mut None, "", parse_col_range).err(CRDot).q(R);
        str_parse(&mut None, "'iri'#.A:.C", parse_col_range)
            .ok(iri, "iri")
            .q(R);
        str_parse(&mut None, "'sheet'.A:.C", parse_col_range)
            .ok(sheet_name, "sheet")
            .q(R);
        str_parse(&mut None, ".A:.C", parse_col_range)
            .ok(col_col, &(0, 2))
            .q(R);
        str_parse(&mut None, ".1:", parse_col_range).err(CRCol).q(R);
        str_parse(&mut None, ".A", parse_col_range)
            .err(CRColon)
            .q(R);
        str_parse(&mut None, ":", parse_col_range).err(CRDot).q(R);
        str_parse(&mut None, ":.A", parse_col_range).err(CRDot).q(R);
        str_parse(&mut None, ":A", parse_col_range).err(CRDot).q(R);
        str_parse(&mut None, ".5:.7", parse_col_range)
            .err(CRCol)
            .q(R);
        str_parse(&mut None, "'iri'#'sheet'.$A:.$C", parse_col_range)
            .ok(iri, "iri")
            .ok(sheet_name, "sheet")
            .q(R);
    }

    #[test]
    pub(crate) fn rowrange() {
        fn iri(result: &RowRange, test: &str) -> bool {
            match result.iri() {
                Some(iri) => iri == test,
                None => false,
            }
        }
        fn sheet_name(result: &RowRange, test: &str) -> bool {
            match result.table() {
                Some(table) => table == test,
                None => false,
            }
        }
        fn row_row(result: &RowRange, test: &(u32, u32)) -> bool {
            (result.row(), result.to_row()) == *test
        }

        str_parse(&mut None, "", parse_row_range).err(CRDot).q(R);
        str_parse(&mut None, "'iri'#.1:.3", parse_row_range)
            .ok(iri, "iri")
            .q(R);
        str_parse(&mut None, "'sheet'.1:.3", parse_row_range)
            .ok(sheet_name, "sheet")
            .q(R);
        str_parse(&mut None, ".1:.3", parse_row_range)
            .ok(row_row, &(0, 2))
            .q(R);
        str_parse(&mut None, ".1:", parse_row_range).err(CRDot).q(R);
        str_parse(&mut None, ".1", parse_row_range)
            .err(CRColon)
            .q(R);
        str_parse(&mut None, ":", parse_row_range).err(CRDot).q(R);
        str_parse(&mut None, ":.1", parse_row_range).err(CRDot).q(R);
        str_parse(&mut None, ".C:.E", parse_row_range)
            .err(CRRow)
            .q(R);
        str_parse(&mut None, "'iri'#'sheet'.$1:.$3", parse_row_range)
            .ok(iri, "iri")
            .ok(sheet_name, "sheet")
            .q(R);
    }
}
