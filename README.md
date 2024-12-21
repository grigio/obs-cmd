# obs-cmd - a minimal obs CLI for obs-websocket v5

I was used to [obs-cli](https://github.com/muesli/obs-cli/pull/64) but it doesn't support `obs-websocket` 5

[![release](https://github.com/grigio/obs-cmd/actions/workflows/release.yml/badge.svg)](https://github.com/grigio/obs-cmd/actions/workflows/release.yml)

### Usage

Just a minimal API is supported

```
obs-cmd --help
obs-cmd scene switch <scene>
obs-cmd scene switch @cam-front
obs-cmd scene-collection switch <collection>
obs-cmd scene-item toggle <scene> <item>
obs-cmd toggle-mute Mic/Aux
obs-cmd recording start
obs-cmd recording stop
obs-cmd recording toggle
obs-cmd recording pause
obs-cmd recording resume
obs-cmd recording toggle-pause
obs-cmd recording status
obs-cmd streaming start
obs-cmd virtualcam start
obs-cmd save-screenshot <source> <format> <file_path> [--width WIDTH] [--height HEIGHT] [--compression-quality COMPRESSION_QUALITY]
obs-cmd replay toggle
obs-cmd replay save
obs-cmd replay status
obs-cmd replay last-replay
obs-cmd info
obs-cmd --websocket obsws://localhost:4455/secret info # You can override the default `obsws` url
OBS_WEBSOCKET_URL=obsws://localhost:4455/secret obs-cmd info
```

You can override the websocket URL, which can be found in OBS -> Tools -> WebSocket Server Settings. `localhost` for the hostname will work for most, instead of the full IP address. If you set the password as `secret` you can avoid to specify the `--websocket` argument.

### Installation


### Using the provided Binaries
Download `obs-cmd`, pick the correct binary for your OS, example `obs-cmd-linux-amd64`

https://github.com/grigio/obs-cmd/releases/latest

type in the terminal:

```
chmod +x obs-cmd-linux-amd64 && sudo mv obs-cmd-linux-amd64 /usr/local/bin/obs-cmd
```

### Installing From Source
First ensure that Rust is installed on your system. Clone the repo to your local system:

```
git clone https://github.com/grigio/obs-cmd.git
```

Next `cd` into the cloned directory then at the top-level of the directory run:

```
cargo build --release
```

Once the build is complete you may move the produce binary into an approriate binary location, such as `/usr/local/bin`:

```
sudo cp target/release/obs-cmd /usr/local/bin/obs-cmd
```

### Installing on Arch Linux
The `obs-cmd` package is maintained on the [Arch User Repository](https://aur.archlinux.org/packages/obs-cmd).

Ensure that [rust](https://archlinux.org/packages/extra/x86_64/rust/) is installed for access to `cargo`.

To install `obs-cmd` you can either use an AUR helper of your choice such as [yay](https://aur.archlinux.org/packages/yay) or [aurman](https://aur.archlinux.org/packages/aurman) or download the PKGBUILD directly and use makepkg to produce the finished package:
```
wget https://aur.archlinux.org/cgit/aur.git/snapshot/obs-cmd.tar.gz
```

Untar the .tar.gz file:
```
tar xvzf obs-cmd.tar.gz
```

`cd` into the directory:
```
cd obs-cmd
```

Run makepkg to produce the installable package:
```
makepkg -s
```

And finally use pacman to install the produced package (*your version number may vary*):
```
sudo pacman -U obs-cmd-0.15.3-1-x86_64.pkg.tar.zst
```


### Example Usage
```
$ obs-cmd recording start
Recording started
Result: Ok(())
$ obs-cmd recording stop

$ obs-cmd info
Version: Version { obs_version: Version { major: 29, minor: 1, patch: 1 }, obs_web_socket_version: Version { major: 5, minor: 2, patch: 2 }, rpc_version: 1, available_requests: ..

$ obs-cmd save-screenshot "OBS Source" "jpg" "/home/user/screenshot/test.jpg" --width 1920 --height 1080 --compression-quality 100
Saved screenshot to path: "/home/user/screenshot/test.jpg"
```

## Donations

Donations are welcome and will go towards further development of this project

```
monero:88LyqYXn4LdCVDtPWKuton9hJwbo8ZduNEGuARHGdeSJ79BBYWGpMQR8VGWxGDKtTLLM6E9MJm8RvW9VMUgCcSXu19L9FSv
bitcoin:bc1q6mh77hfv8x8pa0clzskw6ndysujmr78j6se025
lightning:techonsapevole@getalby.com
```
