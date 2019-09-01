use crate::style::values::specified::CSSFloat;
use cssparser::{Parser, ParseError, Token};
use crate::style::StyleParseErrorKind;

/// A `<length-percentage>` value. This can be either a `<length>`, a
/// `<percentage>`, or a combination of both via `calc()`.
///
/// https://drafts.csswg.org/css-values-4/#typedef-length-percentage
#[derive(Clone, Debug, PartialEq)]
pub enum LengthPercentage {
    Length(NoCalcLength),
}

impl LengthPercentage {
    pub fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, StyleParseErrorKind<'i>>> {
        let location = input.current_source_location();
        let token = input.next()?;
        match *token {
            Token::Dimension {
                value, ref unit, ..
            } => {
                return NoCalcLength::parse_dimension(value, unit)
                    .map(LengthPercentage::Length)
                    .map_err(|()| location.new_unexpected_token_error(token.clone()));
            },
            _ => return Err(location.new_unexpected_token_error(token.clone())),
        }

        return Err(location.new_unexpected_token_error(token.clone()))
    }
}

/// A `<length>` without taking `calc` expressions into account
///
/// <https://drafts.csswg.org/css-values/#lengths>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoCalcLength {
    /// An absolute length
    ///
    /// <https://drafts.csswg.org/css-values/#absolute-length>
    Absolute(AbsoluteLength),
}

impl NoCalcLength {
    /// Parse a given absolute or relative dimension.
    pub fn parse_dimension(
        value: CSSFloat,
        unit: &str,
    ) -> Result<Self, ()> {
        Ok(match_ignore_ascii_case! { unit,
            "px" => NoCalcLength::Absolute(AbsoluteLength::Px(value)),
            "in" => NoCalcLength::Absolute(AbsoluteLength::In(value)),
            "cm" => NoCalcLength::Absolute(AbsoluteLength::Cm(value)),
            "mm" => NoCalcLength::Absolute(AbsoluteLength::Mm(value)),
            "q" => NoCalcLength::Absolute(AbsoluteLength::Q(value)),
            "pt" => NoCalcLength::Absolute(AbsoluteLength::Pt(value)),
            "pc" => NoCalcLength::Absolute(AbsoluteLength::Pc(value)),
            _ => return Err(())
        })
    }
}

/// Represents an absolute length with its unit
/// <https://drafts.csswg.org/css-values/#absolute-length>
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AbsoluteLength {
    /// An absolute length in pixels (px)
    Px(CSSFloat),
    /// An absolute length in inches (in)
    In(CSSFloat),
    /// An absolute length in centimeters (cm)
    Cm(CSSFloat),
    /// An absolute length in millimeters (mm)
    Mm(CSSFloat),
    /// An absolute length in quarter-millimeters (q)
    Q(CSSFloat),
    /// An absolute length in points (pt)
    Pt(CSSFloat),
    /// An absolute length in pica (pc)
    Pc(CSSFloat),
}
