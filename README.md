# obs-cmd - a minimal obs CLI for obs-websocket v5

I was used to [obs-cli](https://github.com/muesli/obs-cli/pull/64) but it doesn't support `obs-websocket` 5

[![release](https://github.com/grigio/obs-cmd/actions/workflows/release.yml/badge.svg)](https://github.com/grigio/obs-cmd/actions/workflows/release.yml)

### Usage

Just a minimal API is supported

```
obs-cmd scene switch <scene>
obs-cmd scene switch @cam-front
obs-cmd toggle-mute Mic/Aux
obs-cmd recording toggle
obs-cmd streaming start
obs-cmd virtualcam start
obs-cmd replay toggle
obs-cmd replay save
obs-cmd info
obs-cmd --obsws obsws://localhost:4455/secret info # You can override the default `obsws` url
```

### Installation 

Download `obs-cmd`, pick the correct binary for your OS, example `obs-cmd-linux-amd64`

https://github.com/grigio/obs-cmd/releases/latest

type in the terminal:

```
chmod +x obs-cmd-linux-amd64 && sudo mv obs-cmd-linux-amd64 /usr/local/bin/obs-cmd
```

Or run this commands to build the binary from source and install it

```
cargo build --release
sudo ln -s $PWD/target/release/obs-cmd /usr/local/bin/
```

### Configuration

Create the config file in `~/.config/obs-cmd.toml`

```toml
# file: ~/.config/obs-cmd.toml

OBS_WS_PASSWORD = "secret"

```

Open OBS Studio and set your `OBS_WS_PASSWORD` in `Tools > Websocker Server Settings`


### Usage

```
➜ obs-cmd recording start 
Recording started
Result: Ok(())
➜ obs-cmd recording stop 

➜ obs-cmd info
Version: Version { obs_version: Version { major: 29, minor: 1, patch: 1 }, obs_web_socket_version: Version { major: 5, minor: 2, patch: 2 }, rpc_version: 1, available_requests: ..
```

### Donations

Donations are welcome and will go towards further development of this project

```
monero:88LyqYXn4LdCVDtPWKuton9hJwbo8ZduNEGuARHGdeSJ79BBYWGpMQR8VGWxGDKtTLLM6E9MJm8RvW9VMUgCcSXu19L9FSv
bitcoin:bc1q6mh77hfv8x8pa0clzskw6ndysujmr78j6se025
lightning:techonsapevole@getalby.com
```