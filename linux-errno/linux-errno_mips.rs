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

	// https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/arch/mips/include/uapi/asm/errno.h?h=v5.19

	/// No message of desired type
	ENOMSG = 35,
	/// Identifier removed
	EIDRM = 36,
	/// Channel number out of range
	ECHRNG = 37,
	/// Level 2 not synchronized
	EL2NSYNC = 38,
	/// Level 3 halted
	EL3HLT = 39,
	/// Level 3 reset
	EL3RST = 40,
	/// Link number out of range
	ELNRNG = 41,
	/// Protocol driver not attached
	EUNATCH = 42,
	/// No CSI structure available
	ENOCSI = 43,
	/// Level 2 halted
	EL2HLT = 44,
	/// Resource deadlock would occur
	EDEADLK = 45,
	/// No record locks available
	ENOLCK = 46,
	/// Invalid exchange
	EBADE = 50,
	/// Invalid request descriptor
	EBADR = 51,
	/// Exchange full
	EXFULL = 52,
	/// No anode
	ENOANO = 53,
	/// Invalid request code
	EBADRQC = 54,
	/// Invalid slot
	EBADSLT = 55,
	/// File locking deadlock error
	EDEADLOCK = 56,
	/// Bad font file format
	EBFONT = 59,
	/// Device not a stream
	ENOSTR = 60,
	/// No data available
	ENODATA = 61,
	/// Timer expired
	ETIME = 62,
	/// Out of streams resources
	ENOSR = 63,
	/// Machine is not on the network
	ENONET = 64,
	/// Package not installed
	ENOPKG = 65,
	/// Object is remote
	EREMOTE = 66,
	/// Link has been severed
	ENOLINK = 67,
	/// Advertise error
	EADV = 68,
	/// Srmount error
	ESRMNT = 69,
	/// Communication error on send
	ECOMM = 70,
	/// Protocol error
	EPROTO = 71,
	/// RFS specific error
	EDOTDOT = 73,
	/// Multihop attempted
	EMULTIHOP = 74,
	/// Not a data message
	EBADMSG = 77,
	/// File name too long
	ENAMETOOLONG = 78,
	/// Value too large for defined data type
	EOVERFLOW = 79,
	/// Name not unique on network
	ENOTUNIQ = 80,
	/// File descriptor in bad state
	EBADFD = 81,
	/// Remote address changed
	EREMCHG = 82,
	/// Can not access a needed shared library
	ELIBACC = 83,
	/// Accessing a corrupted shared library
	ELIBBAD = 84,
	/// .lib section in a.out corrupted
	ELIBSCN = 85,
	/// Attempting to link in too many shared libraries
	ELIBMAX = 86,
	/// Cannot exec a shared library directly
	ELIBEXEC = 87,
	/// Illegal byte sequence
	EILSEQ = 88,
	/// Function not implemented
	ENOSYS = 89,
	/// Too many symbolic links encountered
	ELOOP = 90,
	/// Interrupted system call should be restarted
	ERESTART = 91,
	/// Streams pipe error
	ESTRPIPE = 92,
	/// Directory not empty
	ENOTEMPTY = 93,
	/// Too many users
	EUSERS = 94,
	/// Socket operation on non-socket
	ENOTSOCK = 95,
	/// Destination address required
	EDESTADDRREQ = 96,
	/// Message too long
	EMSGSIZE = 97,
	/// Protocol wrong type for socket
	EPROTOTYPE = 98,
	/// Protocol not available
	ENOPROTOOPT = 99,
	/// Protocol not supported
	EPROTONOSUPPORT = 120,
	/// Socket type not supported
	ESOCKTNOSUPPORT = 121,
	/// Operation not supported on transport endpoint
	EOPNOTSUPP = 122,
	/// Protocol family not supported
	EPFNOSUPPORT = 123,
	/// Address family not supported by protocol
	EAFNOSUPPORT = 124,
	/// Address already in use
	EADDRINUSE = 125,
	/// Cannot assign requested address
	EADDRNOTAVAIL = 126,
	/// Network is down
	ENETDOWN = 127,
	/// Network is unreachable
	ENETUNREACH = 128,
	/// Network dropped connection because of reset
	ENETRESET = 129,
	/// Software caused connection abort
	ECONNABORTED = 130,
	/// Connection reset by peer
	ECONNRESET = 131,
	/// No buffer space available
	ENOBUFS = 132,
	/// Transport endpoint is already connected
	EISCONN = 133,
	/// Transport endpoint is not connected
	ENOTCONN = 134,
	/// Structure needs cleaning
	EUCLEAN = 135,
	/// Not a XENIX named type file
	ENOTNAM = 137,
	/// No XENIX semaphores available
	ENAVAIL = 138,
	/// Is a named type file
	EISNAM = 139,
	/// Remote I/O error
	EREMOTEIO = 140,
	/// Reserved
	EINIT = 141,
	/// Error 142
	EREMDEV = 142,
	/// Cannot send after transport endpoint shutdown
	ESHUTDOWN = 143,
	/// Too many references: cannot splice
	ETOOMANYREFS = 144,
	/// Connection timed out
	ETIMEDOUT = 145,
	/// Connection refused
	ECONNREFUSED = 146,
	/// Host is down
	EHOSTDOWN = 147,
	/// No route to host
	EHOSTUNREACH = 148,
	/// Operation already in progress
	EALREADY = 149,
	/// Operation now in progress
	EINPROGRESS = 150,
	/// Stale file handle
	ESTALE = 151,
	/// AIO operation canceled
	ECANCELED = 158,
	/// No medium found
	ENOMEDIUM = 159,
	/// Wrong medium type
	EMEDIUMTYPE = 160,
	/// Required key not available
	ENOKEY = 161,
	/// Key has expired
	EKEYEXPIRED = 162,
	/// Key has been revoked
	EKEYREVOKED = 163,
	/// Key was rejected by service
	EKEYREJECTED = 164,
	/// Owner died
	EOWNERDEAD = 165,
	/// State not recoverable
	ENOTRECOVERABLE = 166,
	/// Operation not possible due to RF-kill
	ERFKILL = 167,
	/// Memory page has hardware error
	EHWPOISON = 168,
	/// Quota exceeded
	EDQUOT = 1133,
}

/// Operation would block (alias for [EAGAIN])
pub const EWOULDBLOCK: crate::Error = EAGAIN;
