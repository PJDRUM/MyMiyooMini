# MyLauncher Deploy

## Local simulator

Run:

```sh
./run-local.sh
```

ROMs shown in the simulator come from:

```text
static/Roms
```

## Build Miyoo binaries

Run:

```sh
./scripts/build-miyoo.sh
```

This builds:

```text
target/armv7-unknown-linux-gnueabihf/release/allium-launcher
target/armv7-unknown-linux-gnueabihf/release/allium-menu
```

## Create SD-card layout

Run:

```sh
./scripts/package-sd.sh
```

This creates:

```text
dist-sd/
```

Copy the contents of `dist-sd/` to the root of the Miyoo Mini SD card.

Launch from the stock app list:

```text
Apps -> MyLauncher
```

## Current scope

- Launcher/front-end only
- Games-first flow
- Minimal in-game menu: Resume, Quit game
- No firmware, kernel, bootloader, or emulator-core changes
