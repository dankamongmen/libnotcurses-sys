//! `ncvisual_*` reimplemented functions

use crate::{
    Nc, NcError, NcPlane, NcPlaneOptions, NcResult, NcVisual, NcVisualFlags, NcVisualOptions,
};

/// Creates a new plane as prescribed in `opts`, either as a child
/// or the root of a new pile.
///
/// Blits `ncv` to the created plane according to `vopts`.
///
/// If `opts` has a plane, `NCVISUAL_OPTION_CHILDPLANE` must also be supplied.
//
// NOTE: no equivalent method for now.
#[allow(dead_code)]
pub fn ncvisualplane_create<'a>(
    nc: &'a mut Nc,
    opts: &NcPlaneOptions,
    ncv: &mut NcVisual,
    vopts: Option<&NcVisualOptions>,
) -> NcResult<&'a mut NcPlane> {
    // struct ncplane* plane;
    // if (vopts && vopts->n) {
    //     if(vopts->flags & NCVISUAL_OPTION_CHILDPLANE){
    //         return NULL; // the whole point is to create a new plane
    //     }
    //     plane = ncplane_create(vopts->n, opts);
    // } else {
    //     plane = ncpile_create(nc, opts);
    // }
    // if(plane == NULL){
    //     return NULL;
    // }

    let plane: &mut NcPlane;
    if let Some(vo) = vopts {
        if vo.n.is_null() {
            plane = NcPlane::new_pile(nc, opts)?;
        } else if vo.flags & NcVisualFlags::ChildPlane != NcVisualFlags::None {
            return Err(NcError::new_msg("ncvisualplane_create() ERR"));
        } else {
            plane = NcPlane::new_child(unsafe { &mut *vo.n }, opts)?;
        }
    } else {
        plane = NcPlane::new_pile(nc, opts)?;
    }

    // struct ncvisual_options v;
    // if(!vopts){
    //     vopts = &v;
    //     memset(vopts, 0, sizeof(*vopts));
    // }

    let _vopts2: NcVisualOptions;
    let vopts2_ref: &NcVisualOptions;

    if let Some(vo) = vopts {
        vopts2_ref = vo;
    } else {
        _vopts2 = NcVisualOptions::default();
        vopts2_ref = &_vopts2;
    }

    // WIP:

    // vopts->n = plane;
    // if(ncvisual_blit(nc, ncv, vopts) == NULL){
    //     ncplane_destroy(plane);
    //     vopts->n = NULL;
    //     return NULL;
    // }
    // return plane;

    unsafe { ncv.blit(nc, Some(vopts2_ref))? };

    Ok(plane)
}
