// functions already exported by bindgen : 8
// -----------------------------------------
// (W) wrap: 0
// (#) test: 0
// -----------------------------------------
//   ncfdplane_create
//   ncfdplane_destroy
//   ncfdplane_plane
//   ncsubproc_createv,
//   ncsubproc_createvp,
//   ncsubproc_createvpe,
//   ncsubproc_destroy,
//   ncsubproc_plane,

//! from: <https://notcurses.com/notcurses_fds.3.html>
//!
//! These widgets cause a file descriptor to be read until EOF, and written to a
//! scrolling ncplane. The reading will take place in a notcurses-managed
//! context (the particulars of this context are not defined, and should not be
//! depended upon), which will invoke the provided callbacks with the data read.
//!
//! Essentially, they are simply portable interfaces to asynchronous reading,
//! with ncsubproc also providing subprocess management.
//!
//! If ncsubproc_destroy is called before the subprocess has exited, it will be
//! sent a SIGKILL. If ncsubproc_destroy or ncfdplane_destroy is called while a
//! callback is being invoked, the destroy function will block until the
//! callback is done being invoked. If a user callback returns non-0, the
//! calling object will destroy itself. If a user callback calls the relevant
//! destroy function itself, the thread will exit as if non-0 had been returned,
//! and the ncsubproc's resources will at that time be reclaimed.
//!
//! It is essential that the destroy function be called once and only once,
//! whether it is from within the thread's context, or external to that context.

use crate::c_api::ffi;

mod methods;

/// A raw file descriptor, as returned by [`Nc.inputready_fd`] and
/// [`NcDirect.inputready_fd`].
///
/// [`Nc.inputready_fd`]: crate::Nc#method.inputready_fd
/// [`NcDirect.inputready_fd`]: crate::Nc#method.inputready_fd
pub type NcFd = i32;

/// I/O wrapper to dump file descriptor to [`NcPlane`][crate::NcPlane].
///
/// `type in C: ncfdplane (struct)`
pub type NcFdPlane = ffi::ncfdplane;

/// Options struct for [`NcFdPlane`].
///
/// `type in C: ncfdplane_options (struct)`
pub type NcFdPlaneOptions = ffi::ncfdplane_options;

/// [`NcFdPlane`] wrapper with subprocess management.
///
/// `type in C: ncsubproc (struct)`
pub type NcSubproc = ffi::ncsubproc;

/// Options struct for [`NcSubproc`]
///
/// `type in C: ncsubproc_options (struct)`
pub type NcSubprocOptions = ffi::ncsubproc_options;
