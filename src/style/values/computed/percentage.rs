/* This file was taken directly from Servo. https://github.com/servo/servo/blob/master/components/style/values/computed/percentage.rs
Kosmonaut matches Servo's license, MPL 2.0: https://mozilla.org/MPL/2.0/ */

//! Computed percentages.

use crate::style::values::specified::CSSFloat;
use std::fmt;

/// A computed percentage.
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Percentage(pub CSSFloat);

impl Percentage {
    /// 100%
    #[inline]
    pub fn hundred() -> Self {
        Percentage(1.)
    }

    /// Returns the absolute value for this percentage.
    #[inline]
    pub fn abs(&self) -> Self {
        Percentage(self.0.abs())
    }

    /// Clamps this percentage to a non-negative percentage.
    #[inline]
    pub fn clamp_to_non_negative(self) -> Self {
        Percentage(self.0.max(0.))
    }
}
