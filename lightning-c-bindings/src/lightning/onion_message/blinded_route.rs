// This file is Copyright its original authors, visible in version control
// history and in the source files from which this was generated.
//
// This file is licensed under the license available in the LICENSE or LICENSE.md
// file in the root of this repository or, if no such file exists, the same
// license as that which applies to the original source files from which this
// source was automatically generated.

//! Creating blinded routes and related utilities live here.

use alloc::str::FromStr;
use core::ffi::c_void;
use core::convert::Infallible;
use bitcoin::hashes::Hash;
use crate::c_types::*;
#[cfg(feature="no-std")]
use alloc::{vec::Vec, boxed::Box};


use lightning::onion_message::blinded_route::BlindedRoute as nativeBlindedRouteImport;
pub(crate) type nativeBlindedRoute = nativeBlindedRouteImport;

/// Onion messages can be sent and received to blinded routes, which serve to hide the identity of
/// the recipient.
#[must_use]
#[repr(C)]
pub struct BlindedRoute {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeBlindedRoute,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for BlindedRoute {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeBlindedRoute>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the BlindedRoute, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn BlindedRoute_free(this_obj: BlindedRoute) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn BlindedRoute_free_void(this_ptr: *mut c_void) {
	unsafe { let _ = Box::from_raw(this_ptr as *mut nativeBlindedRoute); }
}
#[allow(unused)]
impl BlindedRoute {
	pub(crate) fn get_native_ref(&self) -> &'static nativeBlindedRoute {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeBlindedRoute {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeBlindedRoute {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}

use lightning::onion_message::blinded_route::BlindedHop as nativeBlindedHopImport;
pub(crate) type nativeBlindedHop = nativeBlindedHopImport;

/// Used to construct the blinded hops portion of a blinded route. These hops cannot be identified
/// by outside observers and thus can be used to hide the identity of the recipient.
#[must_use]
#[repr(C)]
pub struct BlindedHop {
	/// A pointer to the opaque Rust object.

	/// Nearly everywhere, inner must be non-null, however in places where
	/// the Rust equivalent takes an Option, it may be set to null to indicate None.
	pub inner: *mut nativeBlindedHop,
	/// Indicates that this is the only struct which contains the same pointer.

	/// Rust functions which take ownership of an object provided via an argument require
	/// this to be true and invalidate the object pointed to by inner.
	pub is_owned: bool,
}

impl Drop for BlindedHop {
	fn drop(&mut self) {
		if self.is_owned && !<*mut nativeBlindedHop>::is_null(self.inner) {
			let _ = unsafe { Box::from_raw(ObjOps::untweak_ptr(self.inner)) };
		}
	}
}
/// Frees any resources used by the BlindedHop, if is_owned is set and inner is non-NULL.
#[no_mangle]
pub extern "C" fn BlindedHop_free(this_obj: BlindedHop) { }
#[allow(unused)]
/// Used only if an object of this type is returned as a trait impl by a method
pub(crate) extern "C" fn BlindedHop_free_void(this_ptr: *mut c_void) {
	unsafe { let _ = Box::from_raw(this_ptr as *mut nativeBlindedHop); }
}
#[allow(unused)]
impl BlindedHop {
	pub(crate) fn get_native_ref(&self) -> &'static nativeBlindedHop {
		unsafe { &*ObjOps::untweak_ptr(self.inner) }
	}
	pub(crate) fn get_native_mut_ref(&self) -> &'static mut nativeBlindedHop {
		unsafe { &mut *ObjOps::untweak_ptr(self.inner) }
	}
	/// When moving out of the pointer, we have to ensure we aren't a reference, this makes that easy
	pub(crate) fn take_inner(mut self) -> *mut nativeBlindedHop {
		assert!(self.is_owned);
		let ret = ObjOps::untweak_ptr(self.inner);
		self.inner = core::ptr::null_mut();
		ret
	}
}
/// Create a blinded route to be forwarded along `node_pks`. The last node pubkey in `node_pks`
/// will be the destination node.
///
/// Errors if less than two hops are provided or if `node_pk`(s) are invalid.
#[must_use]
#[no_mangle]
pub extern "C" fn BlindedRoute_new(mut node_pks: crate::c_types::derived::CVec_PublicKeyZ, keys_manager: &crate::lightning::chain::keysinterface::KeysInterface) -> crate::c_types::derived::CResult_BlindedRouteNoneZ {
	let mut local_node_pks = Vec::new(); for mut item in node_pks.into_rust().drain(..) { local_node_pks.push( { item.into_rust() }); };
	let mut ret = lightning::onion_message::blinded_route::BlindedRoute::new(&local_node_pks[..], keys_manager, secp256k1::global::SECP256K1);
	let mut local_ret = match ret { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::onion_message::blinded_route::BlindedRoute { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { () /*e*/ }).into() };
	local_ret
}

#[no_mangle]
/// Serialize the BlindedRoute object into a byte array which can be read by BlindedRoute_read
pub extern "C" fn BlindedRoute_write(obj: &crate::lightning::onion_message::blinded_route::BlindedRoute) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn BlindedRoute_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeBlindedRoute) })
}
#[no_mangle]
/// Read a BlindedRoute from a byte array, created by BlindedRoute_write
pub extern "C" fn BlindedRoute_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_BlindedRouteDecodeErrorZ {
	let res: Result<lightning::onion_message::blinded_route::BlindedRoute, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::onion_message::blinded_route::BlindedRoute { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
#[no_mangle]
/// Serialize the BlindedHop object into a byte array which can be read by BlindedHop_read
pub extern "C" fn BlindedHop_write(obj: &crate::lightning::onion_message::blinded_route::BlindedHop) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*obj }.get_native_ref())
}
#[no_mangle]
pub(crate) extern "C" fn BlindedHop_write_void(obj: *const c_void) -> crate::c_types::derived::CVec_u8Z {
	crate::c_types::serialize_obj(unsafe { &*(obj as *const nativeBlindedHop) })
}
#[no_mangle]
/// Read a BlindedHop from a byte array, created by BlindedHop_write
pub extern "C" fn BlindedHop_read(ser: crate::c_types::u8slice) -> crate::c_types::derived::CResult_BlindedHopDecodeErrorZ {
	let res: Result<lightning::onion_message::blinded_route::BlindedHop, lightning::ln::msgs::DecodeError> = crate::c_types::deserialize_obj(ser);
	let mut local_res = match res { Ok(mut o) => crate::c_types::CResultTempl::ok( { crate::lightning::onion_message::blinded_route::BlindedHop { inner: ObjOps::heap_alloc(o), is_owned: true } }).into(), Err(mut e) => crate::c_types::CResultTempl::err( { crate::lightning::ln::msgs::DecodeError::native_into(e) }).into() };
	local_res
}
