#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # traitobject
//!
//! Unsafe helpers for working with raw TraitObjects.

use std::mem;

/// Get the data pointer from this trait object.
///
/// Highly unsafe, as there is no information about the type of the data.
pub unsafe fn data<T: ?Sized>(val: *const T) -> *const () {
    *mem::transmute::<*const *const T, *const *const ()>(&val)
}

/// Get the data pointer from this trait object, mutably.
///
/// Highly unsafe, as there is no information about the type of the data.
pub unsafe fn data_mut<T: ?Sized>(mut val: *mut T) -> *mut () {
    *mem::transmute::<*mut *mut T, *mut *mut ()>(&mut val)
}

