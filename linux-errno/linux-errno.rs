// Copyright (c) 2022 John Millikin <john@john-millikin.com>
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH
// REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
// AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
// INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM
// LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR
// OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.
//
// SPDX-License-Identifier: 0BSD

//! This library defines an [Error] struct that represents error numbers
//! returned from Linux system calls.
//!
//! On Linux, error numbers are architecture-specific. The [arch] modules
//! provide access to error numbers for all supported architectures, and the
//! top-level module re-exports error numbers for the current target platform.

#![no_std]

use core::{fmt, num};

/// Type for error numbers returned from Linux system calls.
///
/// The `Error` type implements `PartialEq` for many integer types, and
/// (optionally) with the POSIX error numbers defined in the [`posix-errno`]
/// library.
///
/// [`posix-errno`]: https://crates.io/crates/posix-errno
#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error(num::NonZeroU16);

impl Error {
	/// Create a new error from a raw error number. If outside the permitted
	/// range `[1, 4096)` for Linux error numbers, returns `None`.
	pub const fn new(errno: u16) -> Option<Error> {
		if errno > 0xFFF {
			return errno_out_of_range();
		}
		match num::NonZeroU16::new(errno) {
			Some(n) => Some(Self(n)),
			None => errno_out_of_range(),
		}
	}

	/// Unsafely create a new error from a raw error number, without checking
	/// whether it's within the permitted range for Linux error numbers.
	///
	/// # Safety
	///
	/// The caller must ensure that `0 < errno <= 0xFFF`. In particular, it is
	/// undefined behavior if `errno` is zero.
	#[inline]
	pub const unsafe fn new_unchecked(errno: u16) -> Error {
		Error(num::NonZeroU16::new_unchecked(errno))
	}

	/// Returns the error number as a primitive `u16`.
	#[inline]
	pub const fn get(&self) -> u16 {
		self.0.get()
	}

	/// Returns the error number as a [`NonZeroU16`](num::NonZeroU16).
	#[inline]
	pub const fn get_nonzero(&self) -> num::NonZeroU16 {
		self.0
	}
}

#[cold]
#[inline]
const fn errno_out_of_range() -> Option<Error> {
	None
}

impl From<Error> for u16 {
	#[inline]
	fn from(err: Error) -> u16 {
		err.0.get()
	}
}

impl From<Error> for num::NonZeroU16 {
	#[inline]
	fn from(err: Error) -> num::NonZeroU16 {
		err.0
	}
}

impl From<Error> for u32 {
	#[inline]
	fn from(err: Error) -> u32 {
		err.0.get().into()
	}
}

impl From<Error> for num::NonZeroU32 {
	#[inline]
	fn from(err: Error) -> num::NonZeroU32 {
		err.0.into()
	}
}

impl From<Error> for i32 {
	#[inline]
	fn from(err: Error) -> i32 {
		err.0.get().into()
	}
}

impl From<Error> for num::NonZeroI32 {
	#[inline]
	fn from(err: Error) -> num::NonZeroI32 {
		err.0.into()
	}
}

impl From<Error> for u64 {
	#[inline]
	fn from(err: Error) -> u64 {
		err.0.get().into()
	}
}

impl From<Error> for num::NonZeroU64 {
	#[inline]
	fn from(err: Error) -> num::NonZeroU64 {
		err.0.into()
	}
}

impl From<Error> for i64 {
	#[inline]
	fn from(err: Error) -> i64 {
		err.0.get().into()
	}
}

impl From<Error> for num::NonZeroI64 {
	#[inline]
	fn from(err: Error) -> num::NonZeroI64 {
		err.0.into()
	}
}

impl fmt::Binary for Error {
	#[inline]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(fmt)
	}
}

impl fmt::LowerHex for Error {
	#[inline]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(fmt)
	}
}

impl fmt::UpperHex for Error {
	#[inline]
	fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
		self.0.fmt(fmt)
	}
}

impl PartialEq<i16> for Error {
	#[inline]
	fn eq(&self, other: &i16) -> bool {
		(*other as u16) == self.0.get()
	}
}

impl PartialEq<Error> for i16 {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		(*self as u16) == other.0.get()
	}
}

impl PartialEq<isize> for Error {
	#[inline]
	fn eq(&self, other: &isize) -> bool {
		(*other as usize) == usize::from(self.0.get())
	}
}

impl PartialEq<Error> for isize {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		(*self as usize) == usize::from(other.0.get())
	}
}

impl PartialEq<num::NonZeroI16> for Error {
	#[inline]
	fn eq(&self, other: &num::NonZeroI16) -> bool {
		(other.get() as u16) == self.0.get()
	}
}

impl PartialEq<Error> for num::NonZeroI16 {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		(self.get() as u16) == other.0.get()
	}
}

impl PartialEq<num::NonZeroIsize> for Error {
	#[inline]
	fn eq(&self, other: &num::NonZeroIsize) -> bool {
		(other.get() as usize) == usize::from(self.0.get())
	}
}

impl PartialEq<Error> for num::NonZeroIsize {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		(self.get() as usize) == usize::from(other.0.get())
	}
}

macro_rules! impl_partial_eq {
	($t:ty) => {
		impl PartialEq<$t> for Error {
			#[inline]
			fn eq(&self, other: &$t) -> bool {
				<$t>::from(self.0.get()) == *other
			}
		}

		impl PartialEq<Error> for $t {
			#[inline]
			fn eq(&self, other: &Error) -> bool {
				<$t>::from(other.0.get()) == *self
			}
		}
	};
}

macro_rules! impl_partial_eq_nonzero {
	($t:ty) => {
		impl PartialEq<$t> for Error {
			#[inline]
			fn eq(&self, other: &$t) -> bool {
				<$t>::from(self.0) == *other
			}
		}

		impl PartialEq<Error> for $t {
			#[inline]
			fn eq(&self, other: &Error) -> bool {
				<$t>::from(other.0) == *self
			}
		}
	};
}

impl_partial_eq!(i32);
impl_partial_eq!(i64);
impl_partial_eq!(u16);
impl_partial_eq!(u32);
impl_partial_eq!(u64);
impl_partial_eq!(usize);

impl_partial_eq_nonzero!(num::NonZeroI32);
impl_partial_eq_nonzero!(num::NonZeroI64);
impl_partial_eq_nonzero!(num::NonZeroU16);
impl_partial_eq_nonzero!(num::NonZeroU32);
impl_partial_eq_nonzero!(num::NonZeroU64);
impl_partial_eq_nonzero!(num::NonZeroUsize);

macro_rules! errno_constants {
	( $( $(#[$meta:meta])* $name:ident = $value:literal , )+ ) => {
		use core::fmt;

		$(
			$(#[$meta])*
			pub const $name: $crate::Error = unsafe {
				$crate::Error::new_unchecked($value)
			};
		)*

		#[inline]
		pub(crate) const fn err_name(err: $crate::Error) -> Option<&'static str> {
			match err.0.get() {
			$(
				$value => Some(stringify!($name)),
			)*
				_ => None,
			}
		}
	}
}

#[path = "linux-errno_generic.rs"]
mod arch_generic;

#[path = "linux-errno_alpha.rs"]
mod arch_alpha;

#[path = "linux-errno_mips.rs"]
mod arch_mips;

#[path = "linux-errno_parisc.rs"]
mod arch_parisc;

#[path = "linux-errno_sparc.rs"]
mod arch_sparc;

/// Linux error numbers for specific target architectures.
pub mod arch {
	/// Linux error numbers for the `alpha` architecture.
	#[cfg(any(target_arch = "alpha", doc))]
	pub mod alpha {
		pub use crate::arch_alpha::*;
	}

	/// Linux error numbers for the `arm` and `aarch64` architectures.
	#[cfg(any(
		target_arch = "arm",
		target_arch = "aarch64",
		doc,
	))]
	pub mod arm {
		pub use crate::arch_generic::*;
	}

	/// Linux error numbers for the `m68k` architecture.
	#[cfg(any(target_arch = "m68k", doc))]
	pub mod m68k {
		pub use crate::arch_generic::*;
	}

	/// Linux error numbers for the `mips` and `mips64` architectures.
	#[cfg(any(
		target_arch = "mips",
		target_arch = "mips64",
		doc,
	))]
	pub mod mips {
		pub use crate::arch_mips::*;
	}

	/// Linux error numbers for the `powerpc` and `powerpc64` architectures.
	#[cfg(any(
		target_arch = "powerpc",
		target_arch = "powerpc64",
		doc,
	))]
	pub mod powerpc {
		pub use crate::arch_generic::*;

		/// File locking deadlock error
		pub const EDEADLOCK: crate::Error = unsafe {
			crate::Error::new_unchecked(58)
		};

		#[inline]
		const fn err_name(err: crate::Error) -> Option<&'static str> {
			if err == EDEADLOCK {
				return Some(stringify!("EDEADLOCK"));
			}
			crate::arch_generic::err_name(err)
		}
	}

	/// Linux error numbers for the `parisc` architecture.
	#[cfg(any(target_arch = "parisc", doc))]
	pub mod parisc {
		pub use crate::arch_parisc::*;
	}

	/// Linux error numbers for the `riscv32` and `riscv64` architectures.
	#[cfg(any(
		target_arch = "riscv32",
		target_arch = "riscv64",
		doc,
	))]
	pub mod riscv32 {
		pub use crate::arch_generic::*;
	}

	/// Linux error numbers for the `s390x` architecture.
	#[cfg(any(target_arch = "s390x", doc))]
	pub mod s390x {
		pub use crate::arch_sparc::*;
	}

	/// Linux error numbers for the `sparc` and `sparc64` architectures.
	#[cfg(any(
		target_arch = "sparc",
		target_arch = "sparc64",
		doc,
	))]
	pub mod sparc {
		pub use crate::arch_sparc::*;
	}

	/// Linux error numbers for the `x86` and `x86_64` architectures.
	#[cfg(any(
		target_arch = "x86",
		target_arch = "x86_64",
		doc,
	))]
	pub mod x86 {
		pub use crate::arch_generic::*;
	}
}

#[cfg(target_arch = "alpha")]
use crate::arch::alpha as target;

#[cfg(any(
	target_arch = "arm",
	target_arch = "aarch64",
))]
use crate::arch::arm as target;

#[cfg(any(
	target_arch = "mips",
	target_arch = "mips64",
))]
use crate::arch::mips as target;

#[cfg(any(
	target_arch = "powerpc",
	target_arch = "powerpc64",
))]
use crate::arch::powerpc as target;

#[cfg(target_arch = "parisc")]
use crate::arch::parisc as target;

#[cfg(any(
	target_arch = "riscv32",
	target_arch = "riscv64",
))]
use crate::arch::riscv32 as target;

#[cfg(any(
	target_arch = "sparc",
	target_arch = "sparc64",
))]
use crate::arch::sparc as target;

#[cfg(any(
	target_arch = "x86",
	target_arch = "x86_64",
))]
use crate::arch::x86 as target;

#[doc(inline)]
pub use crate::target::*;

impl fmt::Debug for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match crate::target::err_name(*self) {
			Some(name) => f.write_str(name),
			_ => f.debug_tuple("Error").field(&self.0.get()).finish(),
		}
	}
}

#[cfg(feature = "posix-traits")]
const fn from_posix(err: posix_errno::Error) -> Option<Error> {
	use posix_errno::Error as P;
	match err {
		P::E2BIG           => Some(target::E2BIG),
		P::EACCES          => Some(target::EACCES),
		P::EADDRINUSE      => Some(target::EADDRINUSE),
		P::EADDRNOTAVAIL   => Some(target::EADDRNOTAVAIL),
		P::EAFNOSUPPORT    => Some(target::EAFNOSUPPORT),
		P::EAGAIN          => Some(target::EAGAIN),
		P::EALREADY        => Some(target::EALREADY),
		P::EBADF           => Some(target::EBADF),
		P::EBADMSG         => Some(target::EBADMSG),
		P::EBUSY           => Some(target::EBUSY),
		P::ECANCELED       => Some(target::ECANCELED),
		P::ECHILD          => Some(target::ECHILD),
		P::ECONNABORTED    => Some(target::ECONNABORTED),
		P::ECONNREFUSED    => Some(target::ECONNREFUSED),
		P::ECONNRESET      => Some(target::ECONNRESET),
		P::EDEADLK         => Some(target::EDEADLK),
		P::EDESTADDRREQ    => Some(target::EDESTADDRREQ),
		P::EDOM            => Some(target::EDOM),
		P::EDQUOT          => Some(target::EDQUOT),
		P::EEXIST          => Some(target::EEXIST),
		P::EFAULT          => Some(target::EFAULT),
		P::EFBIG           => Some(target::EFBIG),
		P::EHOSTUNREACH    => Some(target::EHOSTUNREACH),
		P::EIDRM           => Some(target::EIDRM),
		P::EILSEQ          => Some(target::EILSEQ),
		P::EINPROGRESS     => Some(target::EINPROGRESS),
		P::EINTR           => Some(target::EINTR),
		P::EINVAL          => Some(target::EINVAL),
		P::EIO             => Some(target::EIO),
		P::EISCONN         => Some(target::EISCONN),
		P::EISDIR          => Some(target::EISDIR),
		P::ELOOP           => Some(target::ELOOP),
		P::EMFILE          => Some(target::EMFILE),
		P::EMLINK          => Some(target::EMLINK),
		P::EMSGSIZE        => Some(target::EMSGSIZE),
		P::EMULTIHOP       => Some(target::EMULTIHOP),
		P::ENAMETOOLONG    => Some(target::ENAMETOOLONG),
		P::ENETDOWN        => Some(target::ENETDOWN),
		P::ENETRESET       => Some(target::ENETRESET),
		P::ENETUNREACH     => Some(target::ENETUNREACH),
		P::ENFILE          => Some(target::ENFILE),
		P::ENOBUFS         => Some(target::ENOBUFS),
		P::ENODATA         => Some(target::ENODATA),
		P::ENODEV          => Some(target::ENODEV),
		P::ENOENT          => Some(target::ENOENT),
		P::ENOEXEC         => Some(target::ENOEXEC),
		P::ENOLCK          => Some(target::ENOLCK),
		P::ENOLINK         => Some(target::ENOLINK),
		P::ENOMEM          => Some(target::ENOMEM),
		P::ENOMSG          => Some(target::ENOMSG),
		P::ENOPROTOOPT     => Some(target::ENOPROTOOPT),
		P::ENOSPC          => Some(target::ENOSPC),
		P::ENOSR           => Some(target::ENOSR),
		P::ENOSTR          => Some(target::ENOSTR),
		P::ENOSYS          => Some(target::ENOSYS),
		P::ENOTCONN        => Some(target::ENOTCONN),
		P::ENOTDIR         => Some(target::ENOTDIR),
		P::ENOTEMPTY       => Some(target::ENOTEMPTY),
		P::ENOTRECOVERABLE => Some(target::ENOTRECOVERABLE),
		P::ENOTSOCK        => Some(target::ENOTSOCK),
		P::ENOTSUP         => None,
		P::ENOTTY          => Some(target::ENOTTY),
		P::ENXIO           => Some(target::ENXIO),
		P::EOPNOTSUPP      => Some(target::EOPNOTSUPP),
		P::EOVERFLOW       => Some(target::EOVERFLOW),
		P::EOWNERDEAD      => Some(target::EOWNERDEAD),
		P::EPERM           => Some(target::EPERM),
		P::EPIPE           => Some(target::EPIPE),
		P::EPROTO          => Some(target::EPROTO),
		P::EPROTONOSUPPORT => Some(target::EPROTONOSUPPORT),
		P::EPROTOTYPE      => Some(target::EPROTOTYPE),
		P::ERANGE          => Some(target::ERANGE),
		P::EROFS           => Some(target::EROFS),
		P::ESPIPE          => Some(target::ESPIPE),
		P::ESRCH           => Some(target::ESRCH),
		P::ESTALE          => Some(target::ESTALE),
		P::ETIME           => Some(target::ETIME),
		P::ETIMEDOUT       => Some(target::ETIMEDOUT),
		P::ETXTBSY         => Some(target::ETXTBSY),
		P::EWOULDBLOCK     => Some(target::EWOULDBLOCK),
		P::EXDEV           => Some(target::EXDEV),
		_ => None,
	}
}

#[cfg(any(feature = "posix-traits", doc))]
impl PartialEq<posix_errno::Error> for Error {
	#[inline]
	fn eq(&self, other: &posix_errno::Error) -> bool {
		from_posix(*other) == Some(*self)
	}
}

#[cfg(any(feature = "posix-traits", doc))]
impl PartialEq<Error> for posix_errno::Error {
	#[inline]
	fn eq(&self, other: &Error) -> bool {
		from_posix(*self) == Some(*other)
	}
}
