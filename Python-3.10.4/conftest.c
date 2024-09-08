/* confdefs.h */
#define _GNU_SOURCE 1
#define _NETBSD_SOURCE 1
#define __BSD_VISIBLE 1
#define _DARWIN_C_SOURCE 1
#define _PYTHONFRAMEWORK ""
#define _XOPEN_SOURCE 700
#define _XOPEN_SOURCE_EXTENDED 1
#define _POSIX_C_SOURCE 200809L
#define STDC_HEADERS 1
#define HAVE_SYS_TYPES_H 1
#define HAVE_SYS_STAT_H 1
#define HAVE_STDLIB_H 1
#define HAVE_STRING_H 1
#define HAVE_MEMORY_H 1
#define HAVE_STRINGS_H 1
#define HAVE_INTTYPES_H 1
#define HAVE_STDINT_H 1
#define HAVE_UNISTD_H 1
#define __EXTENSIONS__ 1
#define _ALL_SOURCE 1
#define _GNU_SOURCE 1
#define _POSIX_PTHREAD_SEMANTICS 1
#define _TANDEM_SOURCE 1
#define ANDROID_API_LEVEL 24
#define STDC_HEADERS 1
#define HAVE_ASM_TYPES_H 1
#define HAVE_CRYPT_H 1
#define HAVE_DLFCN_H 1
#define HAVE_ERRNO_H 1
#define HAVE_FCNTL_H 1
#define HAVE_GRP_H 1
#define HAVE_LANGINFO_H 1
#define HAVE_LIBINTL_H 1
#define HAVE_PTHREAD_H 1
#define HAVE_SCHED_H 1
#define HAVE_SIGNAL_H 1
#define HAVE_TERMIOS_H 1
#define HAVE_UTIME_H 1
#define HAVE_POLL_H 1
#define HAVE_SYS_EPOLL_H 1
#define HAVE_SYS_POLL_H 1
#define HAVE_SYS_XATTR_H 1
#define HAVE_SYS_FILE_H 1
#define HAVE_SYS_IOCTL_H 1
#define HAVE_SYS_PARAM_H 1
#define HAVE_SYS_RANDOM_H 1
#define HAVE_SYS_SELECT_H 1
#define HAVE_SYS_SENDFILE_H 1
#define HAVE_SYS_SOCKET_H 1
#define HAVE_SYS_STATVFS_H 1
#define HAVE_SYS_STAT_H 1
#define HAVE_SYS_SYSCALL_H 1
#define HAVE_SYS_TIME_H 1
#define HAVE_SYS_TIMES_H 1
#define HAVE_SYS_TYPES_H 1
#define HAVE_SYS_UIO_H 1
#define HAVE_SYS_UN_H 1
#define HAVE_SYS_UTSNAME_H 1
#define HAVE_SYS_WAIT_H 1
#define HAVE_PTY_H 1
#define HAVE_SYS_RESOURCE_H 1
#define HAVE_NETPACKET_PACKET_H 1
#define HAVE_SYSEXITS_H 1
#define HAVE_LINUX_TIPC_H 1
#define HAVE_LINUX_RANDOM_H 1
#define HAVE_SPAWN_H 1
#define HAVE_ALLOCA_H 1
#define HAVE_ENDIAN_H 1
#define HAVE_SYS_ENDIAN_H 1
#define HAVE_SYS_SYSMACROS_H 1
#define HAVE_LINUX_AUXVEC_H 1
#define HAVE_SYS_AUXV_H 1
#define HAVE_LINUX_MEMFD_H 1
#define HAVE_LINUX_WAIT_H 1
#define HAVE_SYS_MMAN_H 1
#define HAVE_SYS_EVENTFD_H 1
#define HAVE_DIRENT_H 1
#define MAJOR_IN_SYSMACROS 1
#define HAVE_NET_IF_H 1
#define HAVE_LINUX_NETLINK_H 1
#define HAVE_LINUX_QRTR_H 1
#define HAVE_LINUX_VM_SOCKETS_H 1
#define HAVE_LINUX_CAN_H 1
#define HAVE_LINUX_CAN_BCM_H 1
#define HAVE_LINUX_CAN_J1939_H 1
#define HAVE_LINUX_CAN_RAW_H 1
#define HAVE_MAKEDEV 1
#define HAVE_HTOLE64 1
#define _LARGEFILE_SOURCE 1
#define _FILE_OFFSET_BITS 64
#if defined(SCO_DS)
#undef _OFF_T
#endif
#define RETSIGTYPE void
/* end confdefs.h.  */
#include <stdio.h>
#ifdef HAVE_SYS_TYPES_H
# include <sys/types.h>
#endif
#ifdef HAVE_SYS_STAT_H
# include <sys/stat.h>
#endif
#ifdef STDC_HEADERS
# include <stdlib.h>
# include <stddef.h>
#else
# ifdef HAVE_STDLIB_H
#  include <stdlib.h>
# endif
#endif
#ifdef HAVE_STRING_H
# if !defined STDC_HEADERS && defined HAVE_MEMORY_H
#  include <memory.h>
# endif
# include <string.h>
#endif
#ifdef HAVE_STRINGS_H
# include <strings.h>
#endif
#ifdef HAVE_INTTYPES_H
# include <inttypes.h>
#endif
#ifdef HAVE_STDINT_H
# include <stdint.h>
#endif
#ifdef HAVE_UNISTD_H
# include <unistd.h>
#endif
int
main ()
{
if (sizeof ((ssize_t)))
	    return 0;
  ;
  return 0;
}
