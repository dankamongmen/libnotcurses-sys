use crate::{
    widgets::{NcMenuOptions, NcMenuSection},
    NcChannels,
};

#[allow(unused_imports)] // for doc comments
use crate::widgets::NcMenu;

/// # `NcMenuOptions` constructors
impl NcMenuOptions {
    /// New NcMenuOptions for [`NcMenu`].
    ///
    /// `sections` must contain at least 1 [`NcMenuSection`].
    pub fn new(sections: &mut [NcMenuSection]) -> Self {
        Self::with_all_args(sections, 0, 0, 0)
    }

    /// New NcMenuOptions for [`NcMenu`], with all args.
    ///
    /// `sections` must contain at least 1 [`NcMenuSection`].
    pub fn with_all_args<CHS1, CHS2>(
        sections: &mut [NcMenuSection],
        style_header: CHS1,
        style_sections: CHS2,
        flags: u64,
    ) -> Self
    where
        CHS1: Into<NcChannels>,
        CHS2: Into<NcChannels>,
    {
        assert![!sections.is_empty()];
        Self {
            // array of 'sectioncount' `MenuSection`s
            sections: sections.as_mut_ptr(),

            //
            sectioncount: sections.len() as i32,

            // styling for header
            headerchannels: style_header.into().into(),

            // styling for sections
            sectionchannels: style_sections.into().into(),

            // flag word of NCMENU_OPTION_*
            flags,
        }
    }
}

/// # `NcMenuOptions` methods
// RETHINK
impl NcMenuOptions {
    /// Returns the styling for the header.
    ///
    /// *(No equivalent C style function)*
    pub fn header_channels(&self) -> NcChannels {
        self.headerchannels.into()
    }

    /// Returns a mutable reference of the styling for the sections.
    ///
    /// *(No equivalent C style function)*
    pub fn header_channels_mut(&mut self) -> &mut crate::c_api::NcChannels_u64 {
        &mut self.headerchannels
    }

    /// Returns the styling for the sections.
    ///
    /// *(No equivalent C style function)*
    pub fn section_channels(&self) -> NcChannels {
        self.sectionchannels.into()
    }

    /// Returns a mutable reference of the styling for the sections.
    ///
    /// *(No equivalent C style function)*
    pub fn section_channels_mut(&mut self) -> &mut crate::c_api::NcChannels_u64 {
        &mut self.sectionchannels
    }
}
