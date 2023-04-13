//!

use core::ptr::{null, null_mut};

use crate::{c_api, NcAlign, NcPlaneFlag, NcPlaneOptions, NcResizeCb};

/// Builder object for [`NcPlaneOptions`].
///
/// Can be constructed by calling [`NcPlaneOptions::builder()`].
///
/// By [*default*] it has the [`Marginalized`] flag already set, alongside `(0, 0)`
/// margins, so that it automatically fills the parent plane.
///
/// [*default*]: NcPlaneOptionsBuilder#method.default
/// [`NcPlaneOptions::builder()`]: NcPlaneOptions#method.builder
/// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
#[derive(Clone)]
pub struct NcPlaneOptionsBuilder {
    pub(crate) y: i32,
    pub(crate) x: i32,
    pub(crate) rows: u32,
    pub(crate) cols: u32,
    // TODO:
    // The void* ‘userptr’ can be retrieved (and reset) later.
    // pub(crate) userptr: mut* c_void,
    //
    // A ‘name’ can be set, used in debugging.
    // pub(crate) name: String,
    pub(crate) resizecb: Option<NcResizeCb>,
    pub(crate) flags: u64,
    pub(crate) margin_b: u32,
    pub(crate) margin_r: u32,
}

mod core_impls {
    use core::fmt;

    #[cfg(not(feature = "std"))]
    use alloc::string::String;

    use super::{NcPlaneFlag, NcPlaneOptions, NcPlaneOptionsBuilder};

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
        /// New `NcPlaneOptionsBuilder` with the [`Marginalized`] flag set.
        ///
        /// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
        fn default() -> Self {
            Self {
                y: 0,
                x: 0,
                rows: 0,
                cols: 0,
                resizecb: None,
                flags: NcPlaneFlag::Marginalized.into(),
                margin_b: 0,
                margin_r: 0,
            }
        }
    }

    impl fmt::Debug for NcPlaneOptionsBuilder {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let resizecb_str =
                if self.resizecb.is_some() { String::from("Some") } else { String::from("None") };
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
}

/// # Constructors
impl NcPlaneOptionsBuilder {
    /// New `NcPlaneOptionsBuilder` with the [`Marginalized`] flag set.
    ///
    /// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
    pub fn new() -> Self {
        Self::default()
    }

    /// New builder from pre-existing options.
    pub fn from_options(options: &NcPlaneOptions) -> Self {
        let mut builder = Self::default(); // Marginalized by default

        if options.is_marginalized() {
            builder = builder.margins(options.margin_b, options.margin_r);
        } else {
            builder = builder.rows_cols(options.rows, options.cols);
        }

        if options.is_fixed() {
            builder = builder.fixed(true);
        }

        if options.resizecb.is_some() {
            builder = builder.resizecb(c_api::ncresizecb_to_rust(options.resizecb));
        }

        if options.is_autogrow() {
            builder = builder.autogrow(true);
        }
        if options.is_vscroll() {
            builder = builder.vscroll(true);
        }

        if options.is_veraligned() {
            builder = builder.valign(options.y);
        } else {
            builder = builder.y(options.y);
        }
        if options.is_horaligned() {
            builder = builder.halign(options.x);
        } else {
            builder = builder.x(options.x);
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

/// # Methods (chainable)
impl NcPlaneOptionsBuilder {
    /// Sets the vertical placement relative to parent plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *y* coordinate and unsets the [`VerAligned`] flag.
    ///
    /// [`VerAligned`]: NcPlaneFlag#associatedconstant.VerAligned
    pub fn y(mut self, y: i32) -> Self {
        self.y = y;
        self.flags &= !NcPlaneFlag::VerAligned;
        self
    }

    /// Sets the horizontal placement relative to parent plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *x* coordinate and unsets the [`HorAligned`] flag.
    ///
    /// [`HorAligned`]: NcPlaneFlag#associatedconstant.HorAligned
    pub fn x(mut self, x: i32) -> Self {
        self.x = x;
        self.flags &= !NcPlaneFlag::HorAligned;
        self
    }

    /// Sets the vertical & horizontal placement relative to parent plane.
    ///
    /// Effect: Sets the *`x`* & *`y`* coordinates and unsets the [`VERALIGNED`]
    /// and [`HORALIGNED`] flags.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// [`VerAligned`]: NcPlaneFlag#associatedconstant.VerAligned
    /// [`HorAligned`]: NcPlaneFlag#associatedconstant.HorAligned
    pub fn yx(mut self, y: i32, x: i32) -> Self {
        self.y = y;
        self.x = x;
        self.flags &= !NcPlaneFlag::VerAligned;
        self.flags &= !NcPlaneFlag::HorAligned;
        self
    }

    /// Sets the vertical alignment.
    ///
    /// Default: *[`NcAlign::Top`]*.
    ///
    /// Effect: Sets the *`v`* alignment and the [`VerAligned`] flag.
    ///
    /// [`VerAligned`]: NcPlaneFlag#associatedconstant.VerAligned
    pub fn valign(mut self, v: impl Into<NcAlign>) -> Self {
        self.y = v.into().into();
        self.flags |= NcPlaneFlag::VerAligned;
        self
    }

    /// Sets the horizontal alignment.
    ///
    /// Default: *[`NcAlign::Left`]*.
    ///
    /// Effect: Sets the *`h`* alignment and the [`HorAligned`] flag.
    ///
    /// [`HorAligned`]: NcPlaneFlag#associatedconstant.HorAligned
    pub fn halign(mut self, h: impl Into<NcAlign>) -> Self {
        self.x = h.into().into();
        self.flags |= NcPlaneFlag::HorAligned;
        self
    }

    /// Sets the vertical & horizontal alignment.
    ///
    /// Default: *`(`[`NcAlign::Top`], [`NcAlign::Left`]`)`*.
    ///
    /// Effect: Sets the *`v` & `h`* alignments and the [`VerAligned`]
    /// & [`HorAligned`] flags.
    ///
    /// [`VerAligned`]: NcPlaneFlag#associatedconstant.VerAligned
    /// [`HorAligned`]: NcPlaneFlag#associatedconstant.HorAligned
    pub fn align(mut self, v: impl Into<NcAlign>, h: impl Into<NcAlign>) -> Self {
        self.y = v.into().into();
        self.x = h.into().into();
        self.flags |= NcPlaneFlag::VerAligned;
        self.flags |= NcPlaneFlag::HorAligned;
        self
    }

    /// Sets the number of rows for the plane.
    ///
    /// Must be >0 when not using `margins`.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: sets the *rows* field and unsets the [`Marginalized`] flag.
    ///
    /// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = rows;
        self.flags &= !NcPlaneFlag::Marginalized;
        self
    }

    /// Sets the number of columns for the plane.
    ///
    /// Must be >0 when not using `margins`.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: sets the *cols* field and unsets the [`Marginalized`] flag.
    ///
    /// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
    pub fn cols(mut self, cols: u32) -> Self {
        self.cols = cols;
        self.flags &= !NcPlaneFlag::Marginalized;
        self
    }

    /// Sets the number of rows & columns for the plane.
    ///
    /// Must be >0 when not using `margins`.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// Effect: sets the *rows* & *cols* fields and unsets the [`Marginalized`]
    /// flag.
    ///
    /// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
    pub fn rows_cols(mut self, rows: u32, cols: u32) -> Self {
        self.rows = rows;
        self.cols = cols;
        self.flags &= !NcPlaneFlag::Marginalized;
        self
    }

    /// Sets the bottom & right margins.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// Effect: sets the `margin_b` & `margin_r` fields and the [`Marginalized`]
    /// flag.
    ///
    /// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
    pub fn margins(mut self, bottom: u32, right: u32) -> Self {
        self.margin_b = bottom;
        self.margin_r = right;
        self.flags |= NcPlaneFlag::Marginalized;
        self
    }

    /// If `true`, the plane will **not** scroll with the parent.
    ///
    /// Default: *false* (scrolls with the parent).
    ///
    /// Effect: (un)sets the [`Fixed`] flag.
    ///
    /// [`Fixed`]: NcPlaneFlag#associatedconstant.Fixed
    pub fn fixed(mut self, fixed: bool) -> Self {
        if fixed {
            self.flags |= NcPlaneFlag::Fixed;
        } else {
            self.flags &= !NcPlaneFlag::Fixed;
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
    /// Effect: (un)sets the [`VScroll`] flag.
    ///
    /// See also: [`AutoGrow`].
    ///
    /// [`set_scrolling(true)`]: crate::NcPlane#method.set_scrolling
    /// [`AutoGrow`]: NcPlaneFlag#associatedconstant.AutoGrow
    /// [`VScroll`]: NcPlaneFlag#associatedconstant.VScroll
    pub fn vscroll(mut self, vscroll: bool) -> Self {
        if vscroll {
            self.flags |= NcPlaneFlag::VScroll;
        } else {
            self.flags &= !NcPlaneFlag::VScroll;
        }
        self
    }

    /// If `true`, the plane will grow automatically.
    ///
    /// Default: *false*.
    ///
    /// Effect: (un)sets the [`AutoGrow`] flag.
    ///
    /// Note that just setting `AutoGrow` makes the `NcPlane` grow to the right,
    /// and setting `AutoGrow` + [`VScroll`] makes the `NcPlane` grow down.
    ///
    /// [`AutoGrow`]: NcPlaneFlag#associatedconstant.AutoGrow
    /// [`VScroll`]: NcPlaneFlag#associatedconstant.VScroll
    pub fn autogrow(mut self, autogrow: bool) -> Self {
        if autogrow {
            self.flags |= NcPlaneFlag::AutoGrow;
        } else {
            self.flags &= !NcPlaneFlag::AutoGrow;
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

/// # Methods (setable)
impl NcPlaneOptionsBuilder {
    /// Sets the vertical placement relative to parent plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *`y`* coordinate and unsets the [`VerAligned`] flag.
    ///
    /// [`VerAligned`]: NcPlaneFlag#associatedconstant.VerAligned
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
        self.flags &= !NcPlaneFlag::VerAligned;
    }

    /// Sets the horizontal placement relative to parent plane.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *`x`* coordinate and unsets the [`HorAligned`] flag.
    ///
    /// [`HorAligned`]: NcPlaneFlag#associatedconstant.HorAligned
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
        self.flags &= !NcPlaneFlag::HorAligned;
    }

    /// Sets the vertical & horizontal placement relative to parent plane.
    ///
    /// Effect: Sets the *`x`* & *`y`* coordinates and unsets the [`VERALIGNED`]
    /// and [`HORALIGNED`] flags.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// [`VerAligned`]: NcPlaneFlag#associatedconstant.VerAligned
    /// [`HorAligned`]: NcPlaneFlag#associatedconstant.HorAligned
    pub fn set_yx(&mut self, y: i32, x: i32) {
        self.y = y;
        self.x = x;
        self.flags &= !NcPlaneFlag::VerAligned;
        self.flags &= !NcPlaneFlag::HorAligned;
    }

    /// Sets the vertical alignment.
    ///
    /// Default: *[`NcAlign::Top`]*.
    ///
    /// Effect: Sets the *`y`* alignment and the [`VerAligned`] flag.
    ///
    /// [`VerAligned`]: NcPlaneFlag#associatedconstant.VerAligned
    pub fn set_valign(&mut self, v: impl Into<NcAlign>) {
        self.y = v.into().into();
        self.flags |= NcPlaneFlag::VerAligned;
    }

    /// Sets the horizontal alignment.
    ///
    /// Default: *[`NcAlign::Left`]*.
    ///
    /// Effect: Sets the *`h`* alignment and the [`HorAligned`] flag.
    ///
    /// [`HorAligned`]: NcPlaneFlag#associatedconstant.HorAligned
    pub fn set_halign(&mut self, h: impl Into<NcAlign>) {
        self.x = h.into().into();
        self.flags |= NcPlaneFlag::HorAligned;
    }

    /// Sets the vertical & horizontal alignment.
    ///
    /// Default: *`(`[`NcAlign::Top`], [`NcAlign::Left`]`)`*.
    ///
    /// Effect: Sets the *`v` & `h`* alignments and the [`VerAligned`]
    /// & [`HorAligned`] flags.
    ///
    /// [`VerAligned`]: NcPlaneFlag#associatedconstant.VerAligned
    /// [`HorAligned`]: NcPlaneFlag#associatedconstant.HorAligned
    pub fn set_align(&mut self, v: impl Into<NcAlign>, h: impl Into<NcAlign>) {
        self.y = v.into().into();
        self.x = h.into().into();
        self.flags |= NcPlaneFlag::VerAligned;
        self.flags |= NcPlaneFlag::HorAligned;
    }

    /// Sets the number of rows for the plane.
    ///
    /// Must be >0 when not using `margins`.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: sets the *rows* field and unsets the [`Marginalized`] flag.
    ///
    /// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
    pub fn set_rows(&mut self, rows: u32) {
        self.rows = rows;
        self.flags &= !NcPlaneFlag::Marginalized;
    }

    /// Sets the number of columns for the plane.
    ///
    /// Must be >0 when not using `margins`.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: sets the *cols* field and unsets the [`Marginalized`] flag.
    ///
    /// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
    pub fn set_cols(&mut self, cols: u32) {
        self.cols = cols;
        self.flags &= !NcPlaneFlag::Marginalized;
    }

    /// Sets the number of rows & columns for the plane.
    ///
    /// Must be >0 when not using `margins`.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// Effect: sets the *rows* & *cols* fields and unsets the [`Marginalized`]
    /// flag.
    ///
    /// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
    pub fn set_rows_cols(&mut self, rows: u32, cols: u32) {
        self.rows = rows;
        self.cols = cols;
        self.flags &= !NcPlaneFlag::Marginalized;
    }

    /// Sets the bottom & right margins.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// Effect: sets the `margin_b` & `margin_r` fields and the [`Marginalized`]
    /// flag.
    ///
    /// [`Marginalized`]: NcPlaneFlag#associatedconstant.Marginalized
    pub fn set_margins(&mut self, bottom: u32, right: u32) {
        self.margin_b = bottom;
        self.margin_r = right;
        self.flags |= NcPlaneFlag::Marginalized;
    }

    /// If `true`, the plane will **not** scroll with the parent.
    ///
    /// Default: *false* (scrolls with the parent).
    ///
    /// Effect: (un)sets the [`Fixed`] flag.
    ///
    /// [`Fixed`]: NcPlaneFlag#associatedconstant.Fixed
    pub fn set_fixed(&mut self, fixed: bool) {
        if fixed {
            self.flags |= NcPlaneFlag::Fixed;
        } else {
            self.flags &= !NcPlaneFlag::Fixed;
        }
    }

    /// If `true`, the plane will scroll vertically to accommodate output.
    ///
    /// Setting this flag is equivalent to immediately calling
    /// [`set_scrolling(true)`] following `NcPlane` creation.
    ///
    /// Default: *false*.
    ///
    /// Effect: (un)sets the [`VScroll`] flag.
    ///
    /// See also: [`AutoGrow`].
    ///
    /// [`set_scrolling(true)`]: crate::NcPlane#method.set_scrolling
    /// [`AutoGrow`]: NcPlaneFlag#associatedconstant.AutoGrow
    /// [`VScroll`]: NcPlaneFlag#associatedconstant.VScroll
    pub fn set_vscroll(&mut self, vscroll: bool) {
        if vscroll {
            self.flags |= NcPlaneFlag::VScroll;
        } else {
            self.flags &= !NcPlaneFlag::VScroll;
        }
    }

    /// If `true`, the plane will grow automatically.
    ///
    /// Default: *false*.
    ///
    /// Effect: (un)sets the [`AutoGrow`] flag.
    ///
    /// Note that just setting `AutoGrow` makes the `NcPlane` grow to the right,
    /// and setting `AutoGrow` + [`VScroll`] makes the `NcPlane` grow down.
    ///
    /// [`AutoGrow`]: NcPlaneFlag#associatedconstant.AutoGrow
    /// [`VScroll`]: NcPlaneFlag#associatedconstant.VScroll
    pub fn set_autogrow(&mut self, autogrow: bool) {
        if autogrow {
            self.flags |= NcPlaneFlag::AutoGrow;
        } else {
            self.flags &= !NcPlaneFlag::AutoGrow;
        }
    }

    /// (Un)Sets the resize callback.
    ///
    /// Default: *None*.
    pub fn set_resizecb(&mut self, callback: Option<NcResizeCb>) {
        self.resizecb = callback;
    }
}
