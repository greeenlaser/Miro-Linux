# Miro on Linux

The purpose of this program is to bring Miro to Linux (targeted to Arch and Arch-likes but should work on all others) and to allow using [miro](httpos://miro.com) natively on Linux via a Desktop app because they haven't done one so far for whatever reason.

Some users may encounter the error "Error 71 (Protocol error) dispatching to Wayland display.". The fix for that is to run Miro like this: WEBKIT_DISABLE_DMABUF_RENDERER=1 ./Miro

You will need [Tauri](https://v2.tauri.app/) and a few other dependencies that have been listed in the[quick setup doc](quick-setup.md) if you wish to modify this program and its source code yourself.

The developers and maintainers of this program are not affiliated in any way with Miro, this is simply an unofficial project for personal use.
