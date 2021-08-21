# tenki-slack-post.rs

[![Actions Status](https://github.com/thekuwayama/tenki-slack-post.rs/workflows/CI/badge.svg)](https://github.com/thekuwayama/tenki-slack-post.rs/actions/workflows/main.yml)
[![MIT licensed](https://img.shields.io/badge/license-MIT-brightgreen.svg)](https://raw.githubusercontent.com/thekuwayama/tenki-slack-post.rs/main/LICENSE-MIT)
[![Apache licensed](https://img.shields.io/badge/license-Apache-brightgreen.svg)](https://raw.githubusercontent.com/thekuwayama/tenki-slack-post.rs/main/LICENSE-APACHE)

`tenki-slack-post.rs` is CLI that posts a rain radar image to Slack.
It gets the image from tenki.jp.


## Install

You can build and run `tenki-slack-post.rs` with the following:

```sh-session
$ cargo install --git https://github.com/thekuwayama/tenki-slack-post.rs.git --branch main
```


## Usage

```sh-session
$ tenki_slack_post --help
tenki_slack_post 0.1.0


USAGE:
    tenki_slack_post <oauth-token> <channel>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <oauth-token>    Slack OAuth Token
    <channel>        Slack Channel Name
```

```sh-session
$ tenki_slack_post xoxp-xxxxxxxxxxxx-xxxxxxxxxxxx-xxxxxxxxxxxxx-xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx "#channel_name"
```

![post](post.png)


## License

The CLI is available as open source under the terms of the [Apache-2.0 License](https://opensource.org/licenses/Apache-2.0) or [MIT License](http://opensource.org/licenses/MIT).
