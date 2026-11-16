//! Zero-copy interop between molrs Python objects and molrs-ffi handles.
//!
//! molpy and molpack are **separate** PyO3 extensions, so a `molpy.core.Frame`
//! pyclass cannot be `.extract()`d into a Rust value here. Instead molpy
//! exposes a stable-FFI capsule — `Frame._ffi_frameref_capsule()` — and molpack
//! resolves it to the shared `molrs_ffi::FrameRef` handle. This is the exact
//! pattern molrs-cxxapi uses (`frame_clone_from_addr`): **no dict marshalling,
//! no consumer-side data type** (there is no `mpk.Frame`).
//!
//! Soundness: both wheels link the same `molcrafts-molrs-ffi` and the same
//! always-on `molcrafts-molrs` core, whose `Frame` / `Block` / `SimBox` layout
//! is feature-independent. So a handle minted by molpy (built with the
//! `full` feature set) and the `molrs::core::Frame` it lends are layout-identical to
//! what molpack (built on molrs's always-on core alone) sees across the
//! extension boundary.
//!
//! The capsule names come from `molrs_ffi::abi` and carry the handle-layout
//! fingerprint of the molrs build that minted them, so a capsule from a wheel
//! built from another molpy source or Rust toolchain fails the name check
//! instead of dereferencing a drifted layout.
//!
//! The same pattern carries regions: every molrs region object exports
//! `_ffi_regionref_capsule()`, and
//! [`region_from_py`] resolves it to the shared `Arc<dyn Region>` that
//! `RegionRestraint` evaluates. Unlike a frame, a region is `Send + Sync` and
//! is shared straight into the rayon evaluator.

use std::sync::Arc;

use molrs::core::Frame;
use molrs::core::Region;
use molrs::core::SimBox;
use molrs_ffi::{FfiError, FrameRef, RegionRef};
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyCapsule, PyModule};

use molrs::op::F;

/// Map a molrs-ffi handle error into a Python exception.
fn ffi_err(e: FfiError) -> PyErr {
    PyTypeError::new_err(format!("molrs FFI error: {e}"))
}

/// Resolve a `molpy.core.Frame` (which `molpy.Frame` is) to a shared [`FrameRef`] (zero-copy).
///
/// Clones the handle the capsule carries (one `Arc` bump) onto the same store,
/// so reads/writes through the returned handle are visible in the originating
/// Python frame. The object must expose `_ffi_frameref_capsule()` — i.e. be a
/// real `molpy.core.Frame` (a plain `dict` is refused).
pub fn frame_from_py(obj: &Bound<'_, PyAny>) -> PyResult<FrameRef> {
    let capsule = capsule_from(obj, "_ffi_frameref_capsule")?;
    // The expected name carries the handle-layout fingerprint of the molrs
    // this wheel embeds, so a producer with another layout fails here cleanly
    // instead of being dereferenced.
    let expected = molrs_ffi::abi::frameref_capsule_name();
    let ptr = capsule
        .pointer_checked(Some(expected))
        .map_err(|err| foreign_capsule_error(err, expected))?;
    let pp = ptr.as_ptr() as *const *const FrameRef;
    // SAFETY: the named capsule's void* is `*mut *mut FrameRef` (the
    // exporter boxes a `*mut FrameRef`); deref twice to reach the cloned handle
    // and `.clone()` it (one Arc bump). The capsule is only touched under the GIL.
    let fref = unsafe { (**pp).clone() };
    Ok(fref)
}

/// Resolve any molrs region object (one exposing `_ffi_regionref_capsule()`)
/// to the shared region behind it.
///
/// The capsule name carries the molrs handle-layout fingerprint, so a region
/// produced by a wheel with another layout fails here with a clear
/// `ValueError` instead of being dereferenced.
pub fn region_from_py(obj: &Bound<'_, PyAny>) -> PyResult<Arc<dyn Region + Send + Sync>> {
    let capsule = capsule_from(obj, "_ffi_regionref_capsule")?;
    let expected = molrs_ffi::abi::regionref_capsule_name();
    let ptr = capsule
        .pointer_checked(Some(expected))
        .map_err(|err| foreign_capsule_error(err, expected))?;
    let pp = ptr.as_ptr() as *const *const RegionRef;
    // SAFETY: the named capsule's void* is `*mut *mut RegionRef` (the
    // exporter boxes a `*mut RegionRef`); deref twice to reach the handle and
    // clone it (one `Arc` bump). The capsule is only touched under the GIL.
    let handle = unsafe { (**pp).clone() };
    Ok(handle.region())
}

/// Resolve a frame-like Python object to an **owned** core [`Frame`], deep-copied
/// out of the shared store.
///
/// The common case for consumers that keep the frame (target assembly, potential
/// compilation) — equivalent to `frame_from_py(obj)?.clone_frame()`.
pub fn owned_frame_from_py(obj: &Bound<'_, PyAny>) -> PyResult<Frame> {
    frame_from_py(obj)?.clone_frame().map_err(ffi_err)
}

/// Stamp an orthorhombic periodic box from `(min, max)` corners onto *frame*.
///
/// No-op when *box_bounds* is `None`. Call once before [`frame_to_py`].
pub fn stamp_box_bounds(frame: &mut Frame, box_bounds: Option<([F; 3], [F; 3])>) -> PyResult<()> {
    let Some((lo, hi)) = box_bounds else {
        return Ok(());
    };
    let lengths = [hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2]];
    let simbox = SimBox::new(molrs::core::Cell::Lengths(lengths), lo, [true, true, true])
        .map_err(|e| PyValueError::new_err(format!("building periodic box: {e}")))?;
    frame.simbox = Some(simbox);
    Ok(())
}

/// One-shot Rust ``Frame`` → Python ``molpy.core.Frame``.
///
/// Two cdylibs cannot share a pyclass, so this clones into a standalone
/// ``FrameRef`` capsule. Do it once and keep the Python object.
pub fn frame_to_py<'py>(py: Python<'py>, frame: &Frame) -> PyResult<Bound<'py, PyAny>> {
    let fref = FrameRef::default();
    fref.with_mut(|slot| *slot = frame.deep_copy())
        .map_err(ffi_err)?;
    let capsule = export_frame_capsule(py, fref)?;
    PyModule::import(py, "molpy.core")?
        .getattr("Frame")?
        .call_method1("_from_ffi_frameref_capsule", (capsule,))
}

/// Box a `FrameRef` into a `molrs_ffi::abi::frameref_capsule_name()` PyCapsule
/// — the exporter side of the return path, mirroring molpy's
/// `Frame._ffi_frameref_capsule`.
fn export_frame_capsule<'py>(py: Python<'py>, fref: FrameRef) -> PyResult<Bound<'py, PyCapsule>> {
    // Same constructor and pointer shape as molpy's
    // ``Frame._ffi_frameref_capsule``: PyO3 boxes the ``FrameRefPtr`` payload,
    // so the capsule void* is ``*mut *mut FrameRef``. The shared abi module is
    // the single source of the (``&'static CStr``) name — never hard-code it.
    let raw = FrameRefPtr(Box::into_raw(Box::new(fref)));
    let name = molrs_ffi::abi::frameref_capsule_name();
    PyCapsule::new_with_value_and_destructor(py, raw, name, |ptr: FrameRefPtr, _ctx| {
        // SAFETY: `ptr.0` came from `Box::into_raw` above and is reclaimed
        // exactly once when the capsule dies.
        drop(unsafe { Box::from_raw(ptr.0) });
    })
}

/// The `ValueError` for a capsule whose name is not the one this wheel's
/// molrs build accepts.
fn foreign_capsule_error(err: PyErr, expected: &std::ffi::CStr) -> PyErr {
    PyValueError::new_err(format!(
        "{err} — molpack accepts capsule name {expected:?}; the producing wheel \
         was built from another molpy source or Rust toolchain. Rebuild \
         molcrafts-molpy and molcrafts-molpack from one molpy checkout."
    ))
}

/// `Send` wrapper around a `*mut FrameRef` for the capsule payload (mirrors
/// molpy's `FrameRefPtr`).
///
/// The boxed `FrameRef` is `Send + Sync` (its store is `Arc<RwLock<_>>`) and
/// owned by the capsule alone; its destructor may reclaim it on any thread.
/// `#[repr(transparent)]` makes the capsule's `void*` a `*mut *mut FrameRef`,
/// the shape `Frame._from_ffi_frameref_capsule` resolves.
#[repr(transparent)]
struct FrameRefPtr(*mut FrameRef);

// SAFETY: the pointee is Send + Sync and uniquely owned — see the type-level doc.
unsafe impl Send for FrameRefPtr {}

/// Call `obj.<method>()` and downcast the result to a `PyCapsule`, with a clear
/// error when the object is not a molrs `Frame` or region.
fn capsule_from<'py>(obj: &Bound<'py, PyAny>, method: &str) -> PyResult<Bound<'py, PyCapsule>> {
    let cap = obj.call_method0(method).map_err(|e| {
        PyTypeError::new_err(format!("expected a molrs object exposing {method}(): {e}"))
    })?;
    cap.cast_into::<PyCapsule>()
        .map_err(|_| PyTypeError::new_err(format!("{method}() did not return a PyCapsule")))
}
