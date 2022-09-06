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

	// https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/arch/parisc/include/uapi/asm/errno.h?h=v5.19

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
	/// Illegal byte sequence
	EILSEQ = 47,
	/// Machine is not on the network
	ENONET = 50,
	/// No data available
	ENODATA = 51,
	/// Timer expired
	ETIME = 52,
	/// Out of streams resources
	ENOSR = 53,
	/// Device not a stream
	ENOSTR = 54,
	/// Package not installed
	ENOPKG = 55,
	/// Link has been severed
	ENOLINK = 57,
	/// Advertise error
	EADV = 58,
	/// Srmount error
	ESRMNT = 59,
	/// Communication error on send
	ECOMM = 60,
	/// Protocol error
	EPROTO = 61,
	/// Multihop attempted
	EMULTIHOP = 64,
	/// RFS specific error
	EDOTDOT = 66,
	/// Not a data message
	EBADMSG = 67,
	/// Too many users
	EUSERS = 68,
	/// Quota exceeded
	EDQUOT = 69,
	/// Stale file handle
	ESTALE = 70,
	/// Object is remote
	EREMOTE = 71,
	/// Value too large for defined data type
	EOVERFLOW = 72,
	/// Invalid exchange
	EBADE = 160,
	/// Invalid request descriptor
	EBADR = 161,
	/// Exchange full
	EXFULL = 162,
	/// No anode
	ENOANO = 163,
	/// Invalid request code
	EBADRQC = 164,
	/// Invalid slot
	EBADSLT = 165,
	/// Bad font file format
	EBFONT = 166,
	/// Name not unique on network
	ENOTUNIQ = 167,
	/// File descriptor in bad state
	EBADFD = 168,
	/// Remote address changed
	EREMCHG = 169,
	/// Can not access a needed shared library
	ELIBACC = 170,
	/// Accessing a corrupted shared library
	ELIBBAD = 171,
	/// .lib section in a.out corrupted
	ELIBSCN = 172,
	/// Attempting to link in too many shared libraries
	ELIBMAX = 173,
	/// Cannot exec a shared library directly
	ELIBEXEC = 174,
	/// Interrupted system call should be restarted
	ERESTART = 175,
	/// Streams pipe error
	ESTRPIPE = 176,
	/// Structure needs cleaning
	EUCLEAN = 177,
	/// Not a XENIX named type file
	ENOTNAM = 178,
	/// No XENIX semaphores available
	ENAVAIL = 179,
	/// Is a named type file
	EISNAM = 180,
	/// Remote I/O error
	EREMOTEIO = 181,
	/// No medium found
	ENOMEDIUM = 182,
	/// Wrong medium type
	EMEDIUMTYPE = 183,
	/// Required key not available
	ENOKEY = 184,
	/// Key has expired
	EKEYEXPIRED = 185,
	/// Key has been revoked
	EKEYREVOKED = 186,
	/// Key was rejected by service
	EKEYREJECTED = 187,
	/// symbol does not exist in executable
	ENOSYM = 215,
	/// Socket operation on non-socket
	ENOTSOCK = 216,
	/// Destination address required
	EDESTADDRREQ = 217,
	/// Message too long
	EMSGSIZE = 218,
	/// Protocol wrong type for socket
	EPROTOTYPE = 219,
	/// Protocol not available
	ENOPROTOOPT = 220,
	/// Protocol not supported
	EPROTONOSUPPORT = 221,
	/// Socket type not supported
	ESOCKTNOSUPPORT = 222,
	/// Operation not supported on transport endpoint
	EOPNOTSUPP = 223,
	/// Protocol family not supported
	EPFNOSUPPORT = 224,
	/// Address family not supported by protocol
	EAFNOSUPPORT = 225,
	/// Address already in use
	EADDRINUSE = 226,
	/// Cannot assign requested address
	EADDRNOTAVAIL = 227,
	/// Network is down
	ENETDOWN = 228,
	/// Network is unreachable
	ENETUNREACH = 229,
	/// Network dropped connection because of reset
	ENETRESET = 230,
	/// Software caused connection abort
	ECONNABORTED = 231,
	/// Connection reset by peer
	ECONNRESET = 232,
	/// No buffer space available
	ENOBUFS = 233,
	/// Transport endpoint is already connected
	EISCONN = 234,
	/// Transport endpoint is not connected
	ENOTCONN = 235,
	/// Cannot send after transport endpoint shutdown
	ESHUTDOWN = 236,
	/// Too many references: cannot splice
	ETOOMANYREFS = 237,
	/// Connection timed out
	ETIMEDOUT = 238,
	/// Connection refused
	ECONNREFUSED = 239,
	/// Remote peer released connection
	EREMOTERELEASE = 240,
	/// Host is down
	EHOSTDOWN = 241,
	/// No route to host
	EHOSTUNREACH = 242,
	/// Operation already in progress
	EALREADY = 244,
	/// Operation now in progress
	EINPROGRESS = 245,
	/// Directory not empty
	ENOTEMPTY = 247,
	/// File name too long
	ENAMETOOLONG = 248,
	/// Too many symbolic links encountered
	ELOOP = 249,
	/// Function not implemented
	ENOSYS = 251,
	///  aio request was canceled before complete (POSIX.4 / HPUX)
	ECANCELLED = 253,
	/// Owner died
	EOWNERDEAD = 254,
	/// State not recoverable
	ENOTRECOVERABLE = 255,
	/// Operation not possible due to RF-kill
	ERFKILL = 256,
	/// Memory page has hardware error
	EHWPOISON = 257,
}

/// SuSv3 and Solaris wants one 'L' (alias for [ECANCELLED])
pub const ECANCELED: crate::Error = ECANCELLED;

/// Alias for [EDEADLK]
pub const EDEADLOCK: crate::Error = EDEADLK;

/// For HP's NFS apparently (alias for [ECONNREFUSED])
pub const EREFUSED: crate::Error = ECONNREFUSED;

/// Operation would block (Not HPUX compliant) (alias for [ECANCELLED])
pub const EWOULDBLOCK: crate::Error = EAGAIN;
