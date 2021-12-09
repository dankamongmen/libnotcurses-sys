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
/// By [*default*] it has the [`MARGINALIZED`] flag already set, alongside `(0, 0)`
/// margins, so that it automatically fills the parent plane.
///
/// [*default*]: NcPlaneOptionsBuilder#method.default
/// [`NcPlaneOptions::builder()`]: NcPlaneOptions#method.builder
/// [`MARGINALIZED`]: NcPlaneOptions#associatedconstant.MARGINALIZED
#[derive(Clone)]
pub struct NcPlaneOptionsBuilder {
    pub(crate) y: NcOffset,
    pub(crate) x: NcOffset,
    pub(crate) rows: NcDim,
    pub(crate) cols: NcDim,
    // TODO:
    // The void* ‘userptr’ can be retrieved (and reset) later.
    // pub(crate) userptr: mut* c_void,
    //
    // A ‘name’ can be set, used in debugging.
    // pub(crate) name: String,
    pub(crate) resizecb: Option<NcResizeCb>,
    pub(crate) flags: u64,
    pub(crate) margin_b: NcDim,
    pub(crate) margin_r: NcDim,
}

//
impl From<NcPlaneOptionsBuilder> for NcPlaneOptions {
    fn from(builder: NcPlaneOptionsBuilder) -> NcPlaneOptions {
        builder.build()
    }
}
impl From<&NcPlaneOptionsBuilder> for NcPlaneOptions {
    fn from(builder: &NcPlaneOptionsBuilder) -> Self {
        builder.clone().build()
    }
}
impl From<&mut NcPlaneOptionsBuilder> for NcPlaneOptions {
    fn from(builder: &mut NcPlaneOptionsBuilder) -> Self {
        builder.clone().build()
    }
}
//
impl From<NcPlaneOptions> for NcPlaneOptionsBuilder {
    fn from(options: NcPlaneOptions) -> NcPlaneOptionsBuilder {
        Self::from_options(&options)
    }
}
impl From<&NcPlaneOptions> for NcPlaneOptionsBuilder {
    fn from(options: &NcPlaneOptions) -> Self {
        Self::from_options(options)
    }
}
impl From<&mut NcPlaneOptions> for NcPlaneOptionsBuilder {
    fn from(options: &mut NcPlaneOptions) -> Self {
        Self::from_options(options)
    }
}

impl Default for NcPlaneOptionsBuilder {
    /// New `NcPlaneOptionsBuilder` with the [`MARGINALIZED`] flag set.
    ///
    /// [`MARGINALIZED`]: NcPlaneOptions#associatedconstant.MARGINALIZED
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

/// # Constructors
impl NcPlaneOptionsBuilder {
    /// New builder from pre-existing options.
    pub fn from_options(options: &NcPlaneOptions) -> Self {
        let mut builder = Self::default(); // MARGINALIZED by default

        // y,x
        if options.is_veraligned() {
            builder = builder.valign(options.y as NcAlign);
        } else {
            builder = builder.y(options.y);
        }
        if options.is_horaligned() {
            builder = builder.halign(options.x as NcAlign);
        } else {
            builder = builder.x(options.x);
        }

        // margins || rows,cols
        if options.is_marginalized() {
            builder = builder.margins(options.margin_b, options.margin_r);
        } else {
            builder = builder.rows_cols(options.rows, options.cols);
        }

        // fixed
        if options.is_fixed() {
            builder = builder.fixed(true);
        }

        // resizecb
        if options.resizecb.is_some() {
            builder = builder.resizecb(c_api::ncresizecb_to_rust(options.resizecb));
        }

        // autogrow
        if options.is_autogrow() {
            builder = builder.autogrow(true);
        }

        // vscroll
        if options.is_vscroll() {
            builder = builder.vscroll(true);
        }

        // TODO: name, userptr

        builder
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

/// # Methods
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
        self.flags |= NcPlaneOptions::HORALIGNED;
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
        self.flags |= NcPlaneOptions::MARGINALIZED;
        self
    }

    /// If `true`, the plane will **not** scroll with the parent.
    ///
    /// Default: *false* (scrolls with the parent).
    ///
    /// Effect: (un)sets the [`FIXED`] flag.
    ///
    /// [`FIXED`]: NcPlaneOptions#associatedconstant.FIXED
    pub fn fixed(mut self, fixed: bool) -> Self {
        if fixed {
            self.flags |= NcPlaneOptions::FIXED;
        } else {
            self.flags &= !NcPlaneOptions::FIXED;
        }
        self
    }

    /// If `true`, the plane will scroll vertically to accommodate output.
    ///
    /// Setting this flag is equivalent to immediately calling
    /// [`set_scrolling(true)`] following `NcPlane` creation.
    ///
    /// Default: *false*.
    ///
    /// Effect: (un)sets the [`VSCROLL`] flag.
    ///
    /// See also: [`AUTOGROW`].
    ///
    /// [`set_scrolling(true)`]: crate::NcPlane#method.set_scrolling
    /// [`AUTOGROW`]: NcPlaneOptions#associatedconstant.AUTOGROW
    /// [`VSCROLL`]: NcPlaneOptions#associatedconstant.VSCROLL
    pub fn vscroll(mut self, vscroll: bool) -> Self {
        if vscroll {
            self.flags |= NcPlaneOptions::VSCROLL;
        } else {
            self.flags &= !NcPlaneOptions::VSCROLL;
        }
        self
    }

    /// If `true`, the plane will grow automatically.
    ///
    /// Default: *false*.
    ///
    /// Effect: (un)sets the [`AUTOGROW`] flag.
    ///
    /// Note that just setting `AUTOGROW` makes the `NcPlane` grow to the right,
    /// and setting `AUTOGROW` + [`VSCROLL`] makes the `NcPlane` grow down.
    ///
    /// [`AUTOGROW`]: NcPlaneOptions#associatedconstant.AUTOGROW
    /// [`VSCROLL`]: NcPlaneOptions#associatedconstant.VSCROLL
    pub fn autogrow(mut self, autogrow: bool) -> Self {
        if autogrow {
            self.flags |= NcPlaneOptions::AUTOGROW;
        } else {
            self.flags &= !NcPlaneOptions::AUTOGROW;
        }
        self
    }

    /// (Un)Sets the resize callback.
    ///
    /// Default: *None*.
    pub fn resizecb(mut self, callback: Option<NcResizeCb>) -> Self {
        self.resizecb = callback;
        self
    }
}
