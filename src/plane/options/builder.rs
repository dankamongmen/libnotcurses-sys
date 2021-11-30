//!

use crate::{c_api, NcAlign, NcDim, NcOffset, NcPlaneOptions, NcResizeCb};

use std::{
    fmt,
    ptr::{null, null_mut},
};

/// Builder object for [`NcPlaneOptions`].
///
/// Can be constructed by calling [`NcPlaneOptions::builder()`].
///
/// By default it already has the [`MARGINALIZED`] flag set, alongside `(0, 0)`
/// margins, so that it automatically fills the parent plane.
///
/// [`NcPlaneOptions::builder()`]: NcPlaneOptions#method.builder
/// [`MARGINALIZED`]: NcPlaneOptions#associatedconstant.MARGINALIZED
pub struct NcPlaneOptionsBuilder {
    y: NcOffset,
    x: NcOffset,
    rows: NcDim,
    cols: NcDim,
    // userptr: , // TODO
    // name: String, // TODO
    resizecb: Option<NcResizeCb>,
    flags: u64,
    margin_b: NcDim,
    margin_r: NcDim,
}

impl Default for NcPlaneOptionsBuilder {
    fn default() -> Self {
        Self {
            y: 0,
            x: 0,
            rows: 0,
            cols: 0,
            resizecb: None,
            flags: NcPlaneOptions::MARGINALIZED,
            margin_b: 0,
            margin_r: 0,
        }
    }
}

impl fmt::Debug for NcPlaneOptionsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let resizecb_str = if self.resizecb.is_some() {
            String::from("Some")
        } else {
            String::from("None")
        };
        f.debug_struct("NcPlaneOptionsBuilder")
            .field("y", &self.y)
            .field("x", &self.x)
            .field("rows", &self.rows)
            .field("cols", &self.cols)
            .field("resizecb", &resizecb_str)
            .field("flags", &self.flags)
            .field("margin_b", &self.margin_b)
            .field("margin_r", &self.margin_r)
            .finish()
    }
}

impl NcPlaneOptionsBuilder {
    /// Sets the vertical placement relative to parent plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *y* coordinate and unsets the [`VERALIGNED`] flag.
    ///
    /// [`VERALIGNED`]: NcPlaneOptions#associatedconstant.VERALIGNED
    pub fn y(mut self, y: NcOffset) -> Self {
        self.y = y;
        self.flags &= !NcPlaneOptions::VERALIGNED;
        self
    }

    /// Sets the horizontal placement relative to parent plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *x* coordinate and unsets the [`HORALIGNED`] flag.
    ///
    /// [`HORALIGNED`]: NcPlaneOptions#associatedconstant.HORALIGNED
    pub fn x(mut self, x: NcOffset) -> Self {
        self.x = x;
        self.flags &= !NcPlaneOptions::HORALIGNED;
        self
    }

    /// Sets the vertical & horizontal placement relative to parent plane.
    ///
    /// Effect: Sets the *`x`* & *`y`* coordinates and unsets the [`VERALIGNED`]
    /// and [`HORALIGNED`] flags.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// [`VERALIGNED`]: NcPlaneOptions#associatedconstant.VERALIGNED
    /// [`HORALIGNED`]: NcPlaneOptions#associatedconstant.HORALIGNED
    pub fn yx(mut self, y: NcOffset, x: NcOffset) -> Self {
        self.y = y;
        self.x = x;
        self.flags &= !NcPlaneOptions::VERALIGNED;
        self.flags &= !NcPlaneOptions::HORALIGNED;
        self
    }

    /// Sets the vertical alignment.
    ///
    /// Default: *[`NcAlign::TOP`]*.
    ///
    /// Effect: Sets the *`y`* alignment and the [`VERALIGNED`] flag.
    ///
    /// [`NcAlign::TOP`]: crate::NcAlign#associatedconstant.TOP
    /// [`VERALIGNED`]: NcPlaneOptions#associatedconstant.VERALIGNED
    pub fn valign(mut self, valign: NcAlign) -> Self {
        self.y = valign as NcOffset;
        self.flags |= NcPlaneOptions::VERALIGNED;
        self
    }

    /// Sets the horizontal alignment.
    ///
    /// Default: *[`NcAlign::LEFT`]*.
    ///
    /// Effect: Sets the *`x`* alignment and the [`HORALIGNED`] flag.
    ///
    /// [`NcAlign::LEFT`]: crate::NcAlign#associatedconstant.LEFT
    /// [`HORALIGNED`]: NcPlaneOptions#associatedconstant.HORALIGNED
    pub fn halign(mut self, halign: NcAlign) -> Self {
        self.y = halign as NcOffset;
        self.flags |= NcPlaneOptions::VERALIGNED;
        self
    }

    /// Sets the vertical & horizontal alignment.
    ///
    /// Default: *`(`[`NcAlign::TOP`], [`NcAlign::LEFT`]`)`*.
    ///
    /// Effect: Sets the *`y` & `x`* alignments and the [`VERALIGNED`]
    /// & [`HORALIGNED`] flags.
    ///
    /// [`NcAlign::LEFT`]: crate::NcAlign#associatedconstant.LEFT
    /// [`NcAlign::TOP`]: crate::NcAlign#associatedconstant.TOP
    /// [`VERALIGNED`]: NcPlaneOptions#associatedconstant.VERALIGNED
    /// [`HORALIGNED`]: NcPlaneOptions#associatedconstant.HORALIGNED
    pub fn align(mut self, halign: NcAlign) -> Self {
        self.y = halign as NcOffset;
        self.flags |= NcPlaneOptions::VERALIGNED;
        self
    }
    /// Sets the bottom & right margins.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// Effect: sets the `margin_b` & `margin_r` fields and the [`MARGINALIZED`]
    /// flag.
    ///
    /// [`MARGINALIZED`]: NcPlaneOptions#associatedconstant.MARGINALIZED
    pub fn margins(mut self, bottom: NcDim, right: NcDim) -> Self {
        self.margin_b = bottom;
        self.margin_r = right;
        self.flags &= !NcPlaneOptions::MARGINALIZED;
        self
    }

    /// Sets the number of rows for the plane.
    ///
    /// Must be >0 when not using `margins`.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: sets the *rows* field and unsets the [`MARGINALIZED`] flag.
    ///
    /// [`MARGINALIZED`]: NcPlaneOptions#associatedconstant.MARGINALIZED
    pub fn rows(mut self, rows: NcDim) -> Self {
        self.rows = rows;
        self.flags &= !NcPlaneOptions::MARGINALIZED;
        self
    }

    /// Sets the number of columns for the plane.
    ///
    /// Must be >0 when not using `margins`.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: sets the *cols* field and unsets the [`MARGINALIZED`] flag.
    ///
    /// [`MARGINALIZED`]: NcPlaneOptions#associatedconstant.MARGINALIZED
    pub fn cols(mut self, cols: NcDim) -> Self {
        self.cols = cols;
        self.flags &= !NcPlaneOptions::MARGINALIZED;
        self
    }

    /// Sets the number of rows & columns for the plane.
    ///
    /// Must be >0 when not using `margins`.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// Effect: sets the *rows* & *cols* fields and unsets the [`MARGINALIZED`]
    /// flag.
    ///
    /// [`MARGINALIZED`]: NcPlaneOptions#associatedconstant.MARGINALIZED
    pub fn rows_cols(mut self, rows: NcDim, cols: NcDim) -> Self {
        self.rows = rows;
        self.cols = cols;
        self.flags &= !NcPlaneOptions::MARGINALIZED;
        self
    }

    /// (Un)Sets the [`NcResizeCb`].
    ///
    /// Default: *none*.
    pub fn resizecb(mut self, callback: Option<NcResizeCb>) -> Self {
        self.resizecb = callback;
        self
    }

    /// Finishes the building and returns [`NcPlaneOptions`].
    pub fn build(self) -> NcPlaneOptions {
        NcPlaneOptions {
            y: self.y,
            x: self.x,
            rows: self.rows,
            cols: self.cols,
            userptr: null_mut(), // TODO
            name: null(),        // TODO
            resizecb: c_api::ncresizecb_to_c(self.resizecb),
            flags: self.flags,
            margin_b: self.margin_b,
            margin_r: self.margin_r,
        }
    }
}
