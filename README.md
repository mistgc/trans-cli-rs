# Trans-cli-rs

## Install
### From Source
```sh
git clone https://github.com/zaiic/trans-cli-rs.git
cd trans-cli-rs
cargo build --release
sudo cp target/release/trans /usr/bin
```

## Usage
```sh
trans 'hello world'
```

## Config
Create a config file in `~/.config/trans-cli-rs/config.toml` and add the following code:

```toml
#config.toml
[basic]
backend = "default"
from = "en"
to = "zh"

[key]
appid = "appid_123"
secret_key = "key_123"
```

The `default` backend is [Baidu Translation](https://fanyi.baidu.com/). The `appid` and the `secret_key` can be acquired on [this site](http://api.fanyi.baidu.com/).

