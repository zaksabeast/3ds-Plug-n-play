# Plug-n-play

Plug-n-play (pronounced 'plugin play' or 'plug and play') is a 3ds sysmodule that runs webassembly plugins.

Plugins can display in-game info that otherwise couldn't be seen and add additional functionality via patches and cheats.

If you'd like to create a simple cheat plugin, it's probably best to use Luma's built in cheat feature. Plug-n-play is geared towards more advanced logic and displaying info.

## Installing and running

Luma 12.0.0+ is required with "Enable loading external FIRMs and modules" option enabled.

1. Download and unzip the latest release
1. Install `pnp_launcher.cia` to your console
1. Copy the cxi files to `sd:/luma/sysmodules/`
   - n3ds should only copy `0004013000CB9702.cxi`
   - o3ds should copy both cxi files
1. Run the pnp launcher from the home menu
1. Start a game and have fun

The first plugin will run automatically. Press "start + down" to see the plugin menu and choose other plugins.

Plugins for all games are loaded from `/pnp/<file>.wasm`. Plugins for specific games are loaded from `/pnp/<title_id>/<file>.wasm`.

## Uninstalling

n3ds + o3ds:

- Delete `/luma/sysmodules/0004013000CB9702.cxi`
- Use FBI to uninstall pnp launcher (0004010000CB9700)

o3ds should also:

- Delete `/luma/sysmodules/0004013000CB9802.cxi`
- Remove the gen 7 mode 3 patch `/luma/titles/<gen 7 title id>/exheader.bin` if it exists

If you used pnp before the cxi change, uninstall the pnp sysmodule and pnp mode 3 sysmodule 0004013000CB9702 and 0004013000CB9802 too.

Be careful not to uninstall the wrong thing!

## Mode 3

2ds/o3ds support for extended memory games is supported, but launching the plugin isn't an ideal experience yet, so a launch method isn't included in the releases. Please open an issue if you'd like to help improve this.

## Icon and home menu sound

If you'd like to contribute a better icon or home menu sound, please open an issue! What exists now is not permanent and I would appreciate if anyone wanted to improve what exists.

## Building

1. Install make, [rust](https://www.rust-lang.org/tools/install), [makerom](https://github.com/3DSGuy/Project_CTR/releases), [ctrtool](https://github.com/3DSGuy/Project_CTR/releases), [devkitarm](https://devkitpro.org/wiki/Getting_Started), and [devkitpro's 3ds libs](https://github.com/devkitPro/docker/blob/2569602fc036110366e1f539e0e5ba7b7a97be57/devkitarm/Dockerfile#L5-L7)
1. Run `make` to test, lint, and build

## Developing the sysmodule

If luma's crash screen is enabled, panics will show the last four characters of the causing file in r9 and the line that caused the panic in r10. While this hides two register's values, it does provide pretty quick insight into a problem area.

## Developing plugins

Plugins are regular webassmebly files, which provides several benefits:

- No special 3ds specific toolchains needed
- Can be written in any language that compiles to wasm
- Easier to write and run unit tests
- Has the potential to run outside a 3ds for faster iterations (e.g. a future web app companion)

The wasm file should be a library with a `void run_frame();` function.

Pnp provides a few functions that the plugin can link to at runtime. These provide functionality such as drawing and accessing buttons pressed by the user. The c abi allows for any supporting language to use them.

Plugins written in rust can use the `pnp_lib` package in this repository to access these functions in a rust-ier way.

## Credits

Thanks to these projects, teams, and individuals for being great resources:

- [3dbrew](https://www.3dbrew.org/), [libctru](https://github.com/devkitPro/libctru/), and [Luma3DS](https://github.com/LumaTeam/Luma3DS) for being great references and providing an easy way to make homebrew
- [devkitPro](https://github.com/devkitPro/) for their toolchain
- [NTR](https://github.com/44670/NTR) for the inspiration and draw functions
- [ShinySylveon04](https://github.com/ShinySylveon04/) for the Plug-n-play name
