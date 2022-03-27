//! `NcMetric`

pub(crate) mod reimplemented {
    use crate::{c_api::ffi, cstring, rstring, NcError, NcResult};

    // TODO: clarify, update and visibilize doc-comments

    /// Takes an arbitrarily large number, and prints it into a fixed-size buffer by
    /// adding the necessary SI suffix.
    ///
    /// Usually, pass a `|NC[IB]PREFIXSTRLEN+1|-sized` buffer to generate up to
    /// `|NC[IB]PREFIXCOLUMNS|` columns' worth of EGCs. The characteristic can
    /// occupy up through `|mult-1|` characters (3 for 1000, 4 for 1024).
    /// The mantissa can occupy either zero or two characters.
    ///
    /// snprintf(3) is used internally, with `s` as its size bound. If the output
    /// requires more size than is available, an error will be returned.
    ///
    /// Floating-point is never used, because an IEEE758 double can only losslessly
    /// represent integers through 2^53-1.
    ///
    /// 2^64-1 is 18446744073709551615, 18.45E(xa). KMGTPEZY thus suffice to handle
    /// an 89-bit uintmax_t. Beyond Z(etta) and Y(otta) lie lands unspecified by SI.
    /// 2^-63 is 0.000000000000000000108, 1.08a(tto).
    ///
    /// val: value to print
    /// s: maximum output size.
    /// decimal: scaling. '1' if none has taken place.
    /// buf: buffer in which string will be generated
    /// omitdec: inhibit printing of all-0 decimal portions
    /// mult: base of suffix system (almost always 1000 or 1024)
    /// uprefix: character to print following suffix ('i' for kibibytes basically).
    /// only printed if suffix is actually printed (input >= mult).
    ///
    /// You are encouraged to consult notcurses_metric(3).
    ///
    // FIXME, remove buf, return String, check mem leaks.
    pub fn ncmetric(
        val: u64,
        s: usize,
        decimal: u64,
        buf: &str,
        omitdec: i32,
        mult: u64,
        uprefix: i32,
    ) -> NcResult<&str> {
        let cbuf = cstring![buf];
        let res =
            unsafe { ffi::ncnmetric(val, s, decimal, cbuf.into_raw(), omitdec, mult, uprefix) };

        if res.is_null() {
            Err(NcError::new_msg(&format![
                "ncmetric({}, {}, {:?}, {}, {}, {})",
                val, s, decimal, omitdec, mult, uprefix
            ]))
        } else {
            Ok(rstring![res])
        }
    }
}

pub(crate) mod c_api {
    use crate::c_api::ffi;

    // The number of columns is one fewer, as the STRLEN expressions must leave
    // an extra byte open in case 'µ' (U+00B5, 0xC2 0xB5) shows up.

    // This is the true number of columns;
    //
    // to set up a printf()-style maximum field width,
    // you should use [IB]PREFIXFMT (see below).
    pub const NCMETRIC_PREFIXCOLUMNS: u32 = ffi::NCPREFIXCOLUMNS;

    // The maximum number of columns used by a mult == 1000 (standard)
    // ncmetric() call.
    pub const NCMETRIC_BPREFIXCOLUMNS: u32 = ffi::NCBPREFIXCOLUMNS;

    // IPREFIXCOLUMNS is the maximum number of columns used by a mult == 1024
    // (digital information) ncmetric().
    pub const NCMETRIC_IPREFIXCOLUMNS: u32 = ffi::NCIPREFIXCOLUMNS;

    //
    // Does not include a '\0' (xxx.xxU)
    pub const NCMETRIC_PREFIXSTRLEN: u32 = ffi::NCPREFIXSTRLEN;

    // The maximum number of columns used by a mult == 1024 call making use of
    // the 'i' suffix.
    // Does not include a '\0' (xxxx.xxUi), i == prefix
    pub const NCMETRIC_BPREFIXSTRLEN: u32 = ffi::NCBPREFIXSTRLEN;

    // Does not include a '\0' (xxxx.xxU)
    pub const NCMETRIC_IPREFIXSTRLEN: u32 = ffi::NCIPREFIXSTRLEN;
}
