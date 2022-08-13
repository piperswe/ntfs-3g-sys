#include <stddef.h>
#include <sys/types.h>
#include <stdarg.h>
#include <time.h>

#define __timespec_defined

#define u8 ntfs_u8
#define u16 ntfs_u16
#define u32 ntfs_u32
#define u64 ntfs_u64

#include <security.h>
#include <layout.h>
#include <acls.h>
#include <attrib.h>
#include <attrlist.h>
#include <bitmap.h>
#include <bootsect.h>
#include <cache.h>
#include <collate.h>
#include <compat.h>
#include <compress.h>
#include <debug.h>
#include <device_io.h>
#include <device.h>
#include <dir.h>
#include <ea.h>
#include <efs.h>
#include <endians.h>
#include <index.h>
#include <inode.h>
#include <ioctl.h>
#include <lcnalloc.h>
#include <logfile.h>
#include <logging.h>
#include <mft.h>
#include <misc.h>
#include <mst.h>
#include <ntfstime.h>
#include <object_id.h>
#include <param.h>
#include <plugin.h>
#include <realpath.h>
#include <reparse.h>
#include <runlist.h>
#include <support.h>
#include <types.h>
#include <unistr.h>
#include <volume.h>
#include <xattrs.h>