//!

use crate::{NcAlign, NcBlitter, NcPlane, NcRgba, NcScale, NcVisualFlag, NcVisualOptions};

/// Builder object for [`NcVisualOptions`].
///
/// Can be constructed by calling [`NcVisualOptions::builder()`].
///
/// [`NcVisualOptions::builder()`]: NcVisualOptions#method.builder
#[derive(Debug, Default)]
pub struct NcVisualOptionsBuilder<'ncplane> {
    plane: Option<&'ncplane mut NcPlane>,
    scale: NcScale,
    y: i32,
    x: i32,
    region_yx_lenyx: Option<(u32, u32, u32, u32)>,
    cell_offset_yx: Option<(u32, u32)>,
    blitter: NcBlitter,
    flags: NcVisualFlag,
    transcolor: NcRgba,
}

impl<'ncplane> NcVisualOptionsBuilder<'ncplane> {
    /// Sets the `NcPlane` where the blitting will be done.
    ///
    /// This `NcPlane` could also be considered the parent of a new plane where
    /// the blitting will occur by utilizing the [`child`] method.
    ///
    /// When no `NcPlane` is provided, one will be created using the exact size
    /// necessary to render the source with perfect fidelity (this might be
    /// smaller or larger than the rendering area).
    ///
    /// Default: *`None`* (no plane).
    ///
    /// See also: *[`parent`]*, *[`child`]*, *[`no_plane`]*.
    ///
    /// [`child`]: NcVisualOptionsBuilder#method.child
    /// [`parent`]: NcVisualOptionsBuilder#method.parent
    /// [`no_plane`]: NcVisualOptionsBuilder#method.no_plane
    pub fn plane(mut self, plane: &'ncplane mut NcPlane) -> Self {
        self.plane = Some(plane);
        self
    }

    /// If true, a [`plane`] must also be provided, which will be the parent
    /// of a new child `NcPlane` into which the blitting will be done.
    ///
    /// If false, the blitting will occur in the provided [`plane`], if any,
    /// or in a newly created `NcPlane` otherwise.
    ///
    /// Default: *false* (no child plaane).
    ///
    /// Effect: Sets the [`CHILDPLANE`] flag.
    ///
    /// See also: *[`plane`]*, *[`parent`]*.
    ///
    /// [`CHILDPLANE`]: NcVisualOptions#associatedconstant.CHILDPLANE
    /// [`plane`]: NcVisualOptionsBuilder#method.plane
    /// [`parent`]: NcVisualOptionsBuilder#method.parent
    pub fn child(mut self, child: bool) -> Self {
        if child {
            self.flags |= NcVisualFlag::ChildPlane;
        } else {
            self.flags &= !NcVisualFlag::ChildPlane;
        }
        self
    }

    /// Sets the `NcPlane` that will be the parent of a new `NcPlane` where
    /// the blitting will be done.
    ///
    /// This is the same as calling both [`plane`] and [`child`].
    ///
    /// See also: *[`plane`]*, *[`child`]*.
    ///
    /// [`plane`]: NcVisualOptionsBuilder#method.plane
    /// [`child`]: NcVisualOptionsBuilder#method.child
    pub fn parent(mut self, plane: &'ncplane mut NcPlane) -> Self {
        self.plane = Some(plane);
        self.flags |= NcVisualFlag::ChildPlane;
        self
    }

    /// Unsets the `NcPlane`.
    ///
    /// Effect: unsets the plane & the [`CHILDPLANE`] flag.
    ///
    /// Default: *`None`* (no plane).
    ///
    /// [`CHILDPLANE`]: NcVisualOptions#associatedconstant.CHILDPLANE
    pub fn no_plane(mut self) -> Self {
        self.plane = None;
        self.flags &= !NcVisualFlag::ChildPlane;
        self
    }

    /// Sets the `NcScale`.
    ///
    /// Default: *[`NcScale::NOSCALE`][crate::NcScale#associatedconstant.NOSCALE]*.
    pub fn scale(mut self, scale: impl Into<NcScale>) -> Self {
        self.scale = scale.into();
        self
    }

    /// Sets the vertical placement.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *y* coordinate, and unsets the [`VerAligned`] flag.
    ///
    /// [`VerAligned`]: NcVisualOptions#associatedconstant.VerAligned
    pub fn y(mut self, y: i32) -> Self {
        self.y = y;
        self.flags &= !NcVisualFlag::VerAligned;
        self
    }

    /// Sets the horizontal placement.
    ///
    /// Default: *`0`*.
    ///
    /// Effect: Sets the *x* coordinate, and unsets the [`HorAligned`] flag.
    ///
    /// [`HorAligned`]: NcVisualOptions#associatedconstant.HorAligned
    pub fn x(mut self, x: i32) -> Self {
        self.x = x;
        self.flags &= !NcVisualFlag::HorAligned;
        self
    }

    /// Sets the vertical & horizontal placement.
    ///
    /// Default: *`(0, 0)`*.
    ///
    /// Effect: Sets the *`y` & `x`* coordinates and unsets the [`VerAligned`]
    /// & [`HorAligned`] flags.
    ///
    /// [`VerAligned`]: NcVisualOptions#associatedconstant.VerAligned
    /// [`HorAligned`]: NcVisualOptions#associatedconstant.HorAligned
    pub fn yx(mut self, y: i32, x: i32) -> Self {
        self.y = y;
        self.x = x;
        self.flags &= !NcVisualFlag::VerAligned;
        self.flags &= !NcVisualFlag::HorAligned;
        self
    }

    /// Sets the vertical alignment.
    ///
    /// Default: *[`NcAlign::Top`]*.
    ///
    /// Effect: Sets the *y* alignment and the [`VerAligned`] flag.
    ///
    /// [`VerAligned`]: NcVisualOptions#associatedconstant.VerAligned
    pub fn valign(mut self, valign: impl Into<NcAlign>) -> Self {
        self.y = valign.into().into();
        self.flags |= NcVisualFlag::VerAligned;
        self
    }

    /// Sets the horizontal alignment.
    ///
    /// Default: *[`NcAlign::Left`]*.
    ///
    /// Effect: Sets the *`x`* alignment and the [`VerAligned`] flag.
    ///
    /// [`VerAligned`]: NcVisualOptions#associatedconstant.VerAligned
    pub fn halign(mut self, halign: impl Into<NcAlign>) -> Self {
        self.x = halign.into().into();
        self.flags |= NcVisualFlag::HorAligned;
        self
    }

    /// Sets the vertical & horizontal alignments.
    ///
    /// Default: *`(`[`NcAlign::Top`]*`, `*[`NcAlign::Left`]`)`*.
    ///
    /// Effect: Sets the *`y` & `x`* alignments and the [`VerAligned`] flag.
    ///
    /// [`VerAligned`]: NcVisualOptions#associatedconstant.VerAligned
    pub fn align(mut self, valign: impl Into<NcAlign>, halign: impl Into<NcAlign>) -> Self {
        self.y = valign.into().into();
        self.x = halign.into().into();
        self.flags |= NcVisualFlag::VerAligned;
        self.flags |= NcVisualFlag::HorAligned;
        self
    }

    /// Choose the `NcBlitter`.
    ///
    /// Default: *[`NcBlitter::Default`]*.
    pub fn blitter(mut self, blitter: impl Into<NcBlitter>) -> Self {
        self.blitter = blitter.into();
        self
    }

    /// Choose [`NcBlitter::Pixel`] for the blitter.
    pub fn pixel(mut self) -> Self {
        self.blitter = NcBlitter::Pixel;
        self
    }

    /// Choose the color to be considered transparent, or `None`.
    ///
    /// Default: *none*.
    ///
    /// Efect: (Un)Sets the transparent color, and the [`ADDALPHA`] flag.
    ///
    /// [`ADDALPHA`]: NcVisualOptions#associatedconstant.ADDALPHA
    pub fn transcolor(mut self, color: Option<impl Into<NcRgba>>) -> Self {
        // if color.is_none() {
        if let Some(color) = color {
            self.transcolor = color.into();
            self.flags |= NcVisualFlag::AddAlpha;
        } else {
            self.flags &= !NcVisualFlag::AddAlpha;
        }
        self
    }

    /// Choose whether to use [`NcAlpha::BLEND`] with the [`NcVisual`], so that
    /// the foreground or background colors can be a composite between
    /// a color and the corresponding colors underneath it.
    ///
    /// Default: *false* (no blend).
    ///
    /// Effect: Sets the [`BLEND`] flag.
    ///
    /// [`BLEND`]: NcVisualOptions#associatedconstant.BLEND
    /// [`NcAlpha::Blend`]: crate::NcAlpha#associatedconstant.BLEND
    /// [`NcVisual`]: crate::NcVisual
    pub fn blend(mut self, blend: bool) -> Self {
        if blend {
            self.flags |= NcVisualFlag::Blend;
        } else {
            self.flags &= !NcVisualFlag::Blend;
        }
        self
    }

    /// Choose between gracefully degrading the blitter, or fail if the choosen
    /// `NcBlitter` is not supported by the terminal.
    ///
    /// Default: *true* (degrades).
    ///
    /// Effect: Sets the [`NODEGRADE`] flag.
    ///
    /// See also: the [*rules of degradation*].
    ///
    /// [`NODEGRADE`]: NcVisualOptions#associatedconstant.NODEGRADE
    /// [*rules of degradation*]: NcBlitter#degradation
    pub fn degrade(mut self, degrade: bool) -> Self {
        if degrade {
            self.flags &= !NcVisualFlag::NoDegrade;
        } else {
            self.flags |= NcVisualFlag::NoDegrade;
        }
        self
    }

    /// Sets the `NOINTERPOLATE` flag.
    ///
    /// Default: *true* (interpolates).
    ///
    pub fn interpolate(mut self, interpolate: bool) -> Self {
        if interpolate {
            self.flags &= !NcVisualFlag::NoInterpolate;
        } else {
            self.flags |= NcVisualFlag::NoInterpolate;
        }
        self
    }

    /// Sets the region to be rendered.
    ///
    /// (start_y, start_x, len_y, len_x)
    pub fn region(mut self, beg_y: u32, beg_x: u32, len_y: u32, len_x: u32) -> Self {
        self.region_yx_lenyx = Some((beg_y, beg_x, len_y, len_x));
        self
    }

    /// Sets the pixel offset within the [`NcCell`][crate::NcCell].
    ///
    ///
    pub fn cell_offset(mut self, y: u32, x: u32) -> Self {
        self.cell_offset_yx = Some((y, x));
        self
    }

    /// Finishes the building and returns [`NcVisualOptions`].
    pub fn build(self) -> NcVisualOptions {
        NcVisualOptions::new(
            self.plane,
            self.scale,
            self.y,
            self.x,
            self.region_yx_lenyx,
            self.cell_offset_yx,
            self.blitter,
            self.flags,
            self.transcolor,
        )
    }
}
