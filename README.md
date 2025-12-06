# obs-cmd

**obs-cmd** is a lightweight, command-line interface for controlling OBS Studio via the obs-websocket v5 protocol. It provides a simple and efficient way to automate your streaming and recording workflows.

[![Release](https://github.com/grigio/obs-cmd/actions/workflows/release.yml/badge.svg)](https://github.com/grigio/obs-cmd/actions/workflows/release.yml)
[![Dependencies status](https://deps.rs/repo/github/grigio/obs-cmd/status.svg)](https://deps.rs/repo/github/grigio/obs-cmd)

## Features

- **Scene Management**: Switch between scenes and scene collections.
- **Recording & Streaming**: Start, stop, and toggle recording and streaming.
- **Source Control**: Toggle filters, mute audio sources, and manage scene items.
- **Camera Control**: Start and stop the virtual camera.
- **Replay Buffer**: Manage the replay buffer, including saving replays.
- **Hotkeys**: Trigger OBS hotkeys by name.
- **Projectors**: Open fullscreen and source projectors.
- **Media Inputs**: Control media playback, including play, pause, and restart.

## Installation

### Binaries

You can download the latest pre-compiled binaries for your operating system from the [Releases](https://github.com/grigio/obs-cmd/releases/latest) page.

**Linux/macOS:**
```bash
# Download the appropriate binary for your system
curl -L https://github.com/grigio/obs-cmd/releases/latest/download/obs-cmd-linux-amd64 -o obs-cmd
chmod +x obs-cmd
sudo mv obs-cmd /usr/local/bin/
```

### From Source

Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed, then run the following commands:

```bash
git clone https://github.com/grigio/obs-cmd.git
cd obs-cmd
cargo install --path .
```

### Arch Linux

`obs-cmd` is available on the Arch User Repository (AUR). You can install it using an AUR helper like `yay`:

```bash
yay -S obs-cmd
```

### Gentoo Linux

`media-video/obs-cmd` is available on [Project:GURU](https://wiki.gentoo.org/wiki/Project:GURU/Information_for_End_Users).

```bash
sudo emerge -av media-video/obs-cmd
```

## Usage

`obs-cmd` connects to the OBS WebSocket server. By default, it attempts to connect to `obsws://localhost:4455` with the password `secret`. You can configure the WebSocket settings in OBS under **Tools → WebSocket Server Settings**.

To override the default connection settings, you can use the `--websocket` flag or set the `OBS_WEBSOCKET_URL` environment variable:

```bash
# Using the --websocket flag
obs-cmd --websocket obsws://<hostname>:<port>/<password> <command>

# Using an environment variable
export OBS_WEBSOCKET_URL=obsws://<hostname>:<port>/<password>
obs-cmd <command>
```

### Examples

```bash
# Switch to a scene named "Live"
obs-cmd scene switch Live

# Start recording
obs-cmd recording start

# Toggle the mute state of an audio source
obs-cmd audio toggle "Mic/Aux"

# Save a screenshot of a source
obs-cmd save-screenshot "Webcam" "png" "/path/to/screenshot.png"
```

For a full list of commands and options, run:
```bash
obs-cmd --help
```

On Linux/GNOME you can map `obs-cmd` commands as global shortcuts.

![Example usage on Linux](https://private-user-images.githubusercontent.com/8074/426738958-51872710-4a28-4437-a1e1-41d1838a5a29.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3NTM0MzI3NDIsIm5iZiI6MTc1MzQzMjQ0MiwicGF0aCI6Ii84MDc0LzQyNjczODk1OC01MTg3MjcxMC00YTI4LTQ0MzctYTFlMS00MWQxODM4YTVhMjkucG5nP1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI1MDcyNSUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNTA3MjVUMDgzNDAyWiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9YmM1ODhlMjBkNjY5MGI3OWM3YWU4NTJhYzBmN2NjOGI4MWViMzIzMzMzNTkxNTM1MDAwOTMzMzU1ZTI0NjY5ZiZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QifQ.kH22xUupaDmjxlQfjiDtXSJtwAvN4Tu1AyCSafxsqR0)

## Linux Packages

[![Packaging status](https://repology.org/badge/vertical-allrepos/obs-cmd.svg)](https://repology.org/project/obs-cmd/versions)


## Donations

If you find this project helpful, please consider making a donation to support its development.

- **Monero**: `88LyqYXn4LdCVDtPWKuton9hJwbo8ZduNEGuARHGdeSJ79BBYWGpMQR8VGWxGDKtTLLM6E9MJm8RvW9VMUgCcSXu19L9FSv`
- **Bitcoin**: `bc1q6mh77hfv8x8pa0clzskw6ndysujmr78j6se025`
