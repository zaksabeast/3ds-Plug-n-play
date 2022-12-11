#include <string.h>
#include <3ds.h>
#include "launch.h"

// Thanks to libctru https://github.com/devkitPro/libctru/blob/09688ea6fc16421041b6dd110ab68bb99ef9df6b/libctru/source/services/pmapp.c#L33
Result _PMAPP_LaunchTitle(Handle pmAppHandle, const FS_ProgramInfo *programInfo, u32 launchFlags)
{
  Result ret = 0;
  u32 *cmdbuf = getThreadCommandBuffer();

  cmdbuf[0] = IPC_MakeHeader(0x1, 5, 0); // 0x10140
  memcpy(&cmdbuf[1], programInfo, sizeof(FS_ProgramInfo));
  cmdbuf[5] = launchFlags;

  if (R_FAILED(ret = svcSendSyncRequest(pmAppHandle)))
    return ret;

  return (Result)cmdbuf[1];
}
