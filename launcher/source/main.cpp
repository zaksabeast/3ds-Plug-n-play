#include <iostream>
#include <3ds.h>
#include "csvc.h"
#include "launch.h"

using namespace std;

Handle pmHandle = 0;

int main()
{
	gfxInitDefault();
	consoleInit(GFX_TOP, NULL);

	printf("Started!\n");

	Result rc = svcControlService(SERVICEOP_STEAL_CLIENT_SESSION, &pmHandle, "pm:app");
	printf("Steal pm:app rc %lx\n", rc);

	const FS_ProgramInfo programInfo = {
			.programId = 0x0004013000CB9702ULL, // pnp
			.mediaType = MEDIATYPE_NAND};

	rc = _PMAPP_LaunchTitle(pmHandle, &programInfo, PMLAUNCHFLAG_LOAD_DEPENDENCIES);
	printf("PMAPP_LaunchTitle rc %lx\n", rc);

	printf("Finished!  Press start to exit.\n");

	// Main loop
	while (aptMainLoop())
	{
		gspWaitForVBlank();
		gfxSwapBuffers();
		hidScanInput();

		u32 kDown = hidKeysDown();
		if (kDown & KEY_START)
			break; // break in order to return to hbmenu
	}

	gfxExit();
	svcCloseHandle(pmHandle);

	return 0;
}
