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

#![allow(unused)]

errno_constants! {
	// https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/uapi/asm-generic/errno-base.h?h=v5.19

	/// Operation not permitted
	EPERM = 1,
	/// No such file or directory
	ENOENT = 2,
	/// No such process
	ESRCH = 3,
	/// Interrupted system call
	EINTR = 4,
	/// I/O error
	EIO = 5,
	/// No such device or address
	ENXIO = 6,
	/// Argument list too long
	E2BIG = 7,
	/// Exec format error
	ENOEXEC = 8,
	/// Bad file number
	EBADF = 9,

	/// No child processes
	ECHILD = 10,
	/// Try again
	EAGAIN = 11,
	/// Out of memory
	ENOMEM = 12,
	/// Permission denied
	EACCES = 13,
	/// Bad address
	EFAULT = 14,
	/// Block device required
	ENOTBLK = 15,
	/// Device or resource busy
	EBUSY = 16,
	/// File exists
	EEXIST = 17,
	/// Cross-device link
	EXDEV = 18,
	/// No such device
	ENODEV = 19,

	/// Not a directory
	ENOTDIR = 20,
	/// Is a directory
	EISDIR = 21,
	/// Invalid argument
	EINVAL = 22,
	/// File table overflow
	ENFILE = 23,
	/// Too many open files
	EMFILE = 24,
	/// Not a typewriter
	ENOTTY = 25,
	/// Text file busy
	ETXTBSY = 26,
	/// File too large
	EFBIG = 27,
	/// No space left on device
	ENOSPC = 28,
	/// Illegal seek
	ESPIPE = 29,

	/// Read-only file system
	EROFS = 30,
	/// Too many links
	EMLINK = 31,
	/// Broken pipe
	EPIPE = 32,
	/// Math argument out of domain of func
	EDOM = 33,
	/// Math result not representable
	ERANGE = 34,

	// https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/arch/sparc/include/uapi/asm/errno.h?h=v5.19

	/// Operation now in progress
	EINPROGRESS = 36,
	/// Operation already in progress
	EALREADY = 37,
	/// Socket operation on non-socket
	ENOTSOCK = 38,
	/// Destination address required
	EDESTADDRREQ = 39,

	/// Message too long
	EMSGSIZE = 40,
	/// Protocol wrong type for socket
	EPROTOTYPE = 41,
	/// Protocol not available
	ENOPROTOOPT = 42,
	/// Protocol not supported
	EPROTONOSUPPORT = 43,
	/// Socket type not supported
	ESOCKTNOSUPPORT = 44,
	/// Op not supported on transport endpoint
	EOPNOTSUPP = 45,
	/// Protocol family not supported
	EPFNOSUPPORT = 46,
	/// Address family not supported by protocol
	EAFNOSUPPORT = 47,
	/// Address already in use
	EADDRINUSE = 48,
	/// Cannot assign requested address
	EADDRNOTAVAIL = 49,

	/// Network is down
	ENETDOWN = 50,
	/// Network is unreachable
	ENETUNREACH = 51,
	/// Net dropped connection because of reset
	ENETRESET = 52,
	/// Software caused connection abort
	ECONNABORTED = 53,
	/// Connection reset by peer
	ECONNRESET = 54,
	/// No buffer space available
	ENOBUFS = 55,
	/// Transport endpoint is already connected
	EISCONN = 56,
	/// Transport endpoint is not connected
	ENOTCONN = 57,
	/// No send after transport endpoint shutdown
	ESHUTDOWN = 58,
	/// Too many references: cannot splice
	ETOOMANYREFS = 59,

	/// Connection timed out
	ETIMEDOUT = 60,
	/// Connection refused
	ECONNREFUSED = 61,
	/// Too many symbolic links encountered
	ELOOP = 62,
	/// File name too long
	ENAMETOOLONG = 63,
	/// Host is down
	EHOSTDOWN = 64,
	/// No route to host
	EHOSTUNREACH = 65,
	/// Directory not empty
	ENOTEMPTY = 66,
	/// SUNOS: Too many processes
	EPROCLIM = 67,
	/// Too many users
	EUSERS = 68,
	/// Quota exceeded
	EDQUOT = 69,

	/// Stale file handle
	ESTALE = 70,
	/// Object is remote
	EREMOTE = 71,
	/// Device not a stream
	ENOSTR = 72,
	/// Timer expired
	ETIME = 73,
	/// Out of streams resources
	ENOSR = 74,
	/// No message of desired type
	ENOMSG = 75,
	/// Not a data message
	EBADMSG = 76,
	/// Identifier removed
	EIDRM = 77,
	/// Resource deadlock would occur
	EDEADLK = 78,
	/// No record locks available
	ENOLCK = 79,

	/// Machine is not on the network
	ENONET = 80,
	/// SunOS: Too many lvls of remote in path
	ERREMOTE = 81,
	/// Link has been severed
	ENOLINK = 82,
	/// Advertise error
	EADV = 83,
	/// Srmount error
	ESRMNT = 84,
	/// Communication error on send
	ECOMM = 85,
	/// Protocol error
	EPROTO = 86,
	/// Multihop attempted
	EMULTIHOP = 87,
	/// RFS specific error
	EDOTDOT = 88,
	/// Remote address changed
	EREMCHG = 89,

	/// Function not implemented
	ENOSYS = 90,
	/// Streams pipe error
	ESTRPIPE = 91,
	/// Value too large for defined data type
	EOVERFLOW = 92,
	/// File descriptor in bad state
	EBADFD = 93,
	/// Channel number out of range
	ECHRNG = 94,
	/// Level 2 not synchronized
	EL2NSYNC = 95,
	/// Level 3 halted
	EL3HLT = 96,
	/// Level 3 reset
	EL3RST = 97,
	/// Link number out of range
	ELNRNG = 98,
	/// Protocol driver not attached
	EUNATCH = 99,

	/// No CSI structure available
	ENOCSI = 100,
	/// Level 2 halted
	EL2HLT = 101,
	/// Invalid exchange
	EBADE = 102,
	/// Invalid request descriptor
	EBADR = 103,
	/// Exchange full
	EXFULL = 104,
	/// No anode
	ENOANO = 105,
	/// Invalid request code
	EBADRQC = 106,
	/// Invalid slot
	EBADSLT = 107,
	/// File locking deadlock error
	EDEADLOCK = 108,
	/// Bad font file format
	EBFONT = 109,

	/// Cannot exec a shared library directly
	ELIBEXEC = 110,
	/// No data available
	ENODATA = 111,
	/// Accessing a corrupted shared library
	ELIBBAD = 112,
	/// Package not installed
	ENOPKG = 113,
	/// Can not access a needed shared library
	ELIBACC = 114,
	/// Name not unique on network
	ENOTUNIQ = 115,
	/// Interrupted syscall should be restarted
	ERESTART = 116,
	/// Structure needs cleaning
	EUCLEAN = 117,
	/// Not a XENIX named type file
	ENOTNAM = 118,
	/// No XENIX semaphores available
	ENAVAIL = 119,

	/// Is a named type file
	EISNAM = 120,
	/// Remote I/O error
	EREMOTEIO = 121,
	/// Illegal byte sequence
	EILSEQ = 122,
	/// Attempt to link in too many shared libs
	ELIBMAX = 123,
	/// .lib section in a.out corrupted
	ELIBSCN = 124,
	/// No medium found
	ENOMEDIUM = 125,
	/// Wrong medium type
	EMEDIUMTYPE = 126,
	/// Operation Cancelled
	ECANCELED = 127,
	/// Required key not available
	ENOKEY = 128,
	/// Key has expired
	EKEYEXPIRED = 129,

	/// Key has been revoked
	EKEYREVOKED = 130,
	/// Key was rejected by service
	EKEYREJECTED = 131,
	/// Owner died
	EOWNERDEAD = 132,
	/// State not recoverable
	ENOTRECOVERABLE = 133,
	/// Operation not possible due to RF-kill
	ERFKILL = 134,
	/// Memory page has hardware error
	EHWPOISON = 135,
}

/// Operation would block (alias for [EAGAIN])
pub const EWOULDBLOCK: crate::Error = EAGAIN;
