use crate::{
    widgets::{NcSelector, NcSelectorItem, NcSelectorOptions},
    NcChannels, NcPlane, NcResult, NcString,
};

/// A handy builder for [`NcSelector`].
///
#[derive(Debug)]
pub struct NcSelectorBuilder {
    title: Option<NcString>,
    secondary: Option<NcString>,
    footer: Option<NcString>,
    items: Vec<(NcString, NcString)>,
    default_item: u32,
    max_display: u32,
    channels: [NcChannels; 5],
    flags: u64,
}

impl Default for NcSelectorBuilder {
    fn default() -> Self {
        Self {
            title: None,
            secondary: None,
            footer: None,
            items: vec![],
            default_item: 0,
            max_display: 0,
            channels: [0, 0, 0, 0, 0],
            flags: 0,
        }
    }
}

impl NcSelectorBuilder {
    /// New `NcSelectorBuilder`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds an item.
    pub fn item(mut self, o: &str, d: &str) -> Self {
        self.items.push((NcString::new(o), NcString::new(d)));
        self
    }

    /// Selects the default item
    ///
    /// It is selected at the start and must be between 0 and itemcount-1.
    //
    // TODO: check when finish it's n-1 at max
    pub fn default_item(mut self, item: u32) -> Self {
        self.default_item = item;
        self
    }

    /// Selects the maximum number of items to display at once.
    ///
    /// 0 uses all available space.
    pub fn max_display(mut self, max: u32) -> Self {
        self.max_display = max;
        self
    }

    /// Sets the title string.
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(NcString::new(title));
        self
    }

    /// Sets the secondary title string.
    pub fn secondary(mut self, secondary: &str) -> Self {
        self.secondary = Some(NcString::new(secondary));
        self
    }

    /// Sets the footer string.
    pub fn footer(mut self, footer: &str) -> Self {
        self.footer = Some(NcString::new(footer));
        self
    }

    /// Sets the flags.
    pub fn flags(mut self, flags: u64) -> Self {
        self.flags = flags;
        self
    }

    /// Sets all the `NcChannels`.
    pub fn all_channels(
        mut self,
        item_opt: NcChannels,
        item_desc: NcChannels,
        seltitle: NcChannels,
        selfooter: NcChannels,
        selbox: NcChannels,
    ) -> Self {
        self.channels = [item_opt, item_desc, seltitle, selfooter, selbox];
        self
    }

    /// Sets the `NcChannels` for the item.
    pub fn item_channels(mut self, opt: NcChannels, desc: NcChannels) -> Self {
        self.channels[0] = opt;
        self.channels[1] = desc;
        self
    }

    /// Sets the `NcChannels` for the title.
    pub fn title_channels(mut self, title: NcChannels) -> Self {
        self.channels[2] = title;
        self
    }

    /// Sets the `NcChannels` for the secondary title and the footer.
    pub fn secondary_channels(mut self, secondary: NcChannels) -> Self {
        self.channels[3] = secondary;
        self
    }

    /// Sets the `NcChannels` for the box title.
    pub fn box_channels(mut self, r#box: NcChannels) -> Self {
        self.channels[4] = r#box;
        self
    }

    /// Finishes the builder and returns the `NcSelector`.
    pub fn finish(self, plane: &mut NcPlane) -> NcResult<&mut NcSelector> {
        let mut selitems = vec![];
        for (o, d) in self.items.iter() {
            selitems.push(NcSelectorItem::new(o, d));
        }
        selitems.push(NcSelectorItem::new_empty());

        let default_item = std::cmp::min(self.default_item, selitems.len() as u32 - 1);

        let options = NcSelectorOptions::with_all_options(
            self.title.as_ref(),
            self.secondary.as_ref(),
            self.footer.as_ref(),
            &selitems,
            default_item,
            self.max_display,
            self.channels[0],
            self.channels[1],
            self.channels[2],
            self.channels[3],
            self.channels[4],
        );

        NcSelector::new(plane, &options)
    }
}
