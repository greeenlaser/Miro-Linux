# Miro on Linux

The purpose of this program is to bring Miro to Linux (targeted to Arch and Arch-likes but should work on all others) and to allow using [miro](httpos://miro.com) natively on Linux via a Desktop app because they haven't done one so far for whatever reason.

The developers and maintainers of this program are not affiliated in any way with Miro, this is simply an unofficial project for personal use.

# If you encounter the Wayland protocol error

Some users may encounter the error "Error 71 (Protocol error) dispatching to Wayland display.". The fix for that is to run Miro like this: WEBKIT_DISABLE_DMABUF_RENDERER=1 ./Miro

# Building from source or modifying the source

This program uses [Tauri](https://v2.tauri.app/) and a few other dependencies that have been listed in the [quick setup doc](quick-setup.md).

To build this app from source simply run 'npm run tauri build' at the root directory of this repository from your shell app.
