[![minimum rustc: 1.88.0](https://img.shields.io/badge/minimum%20rustc-1.88.0-yellowgreen?logo=rust&style=flat-square)](https://www.whatrustisit.com)

Simple docker image to serve static files over HTTP and optionally redirect to HTTPS

## Usage

### Setup

TODO

### Configuration

This image is configured via the following environment variables:

| Variable           | Description                                                                                                                        |
|--------------------|------------------------------------------------------------------------------------------------------------------------------------|
| MOUNT_PATH         | The root URL for serving files from.                                                                                               |
| SERVE_FROM         | The path to the folder on the disk which the files are located in.                                                                 |
| REDIRECT_TO_HTTPS  | If set and non-empty the container will redirect to HTTPS, regardless of current protocol.                                         |
| MAX_URI_CHARACTERS | The maximum length of the URI, in characters, to redirect to HTTPS. Anything over this limit will receive an HTTP 400 Bad Request. |

## Contributing
To enable the pre-commit hook which can run the pipeline checks locally before making a PR, run ```git config core.hooksPath .githooks```

## License

This project is licensed under either of the following licenses, at your option:

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.