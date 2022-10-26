# crf_tyew

An unofficial website for Robocraft 2's CRF.

## Development

I hope you like Rust.

This project uses [yew](https://yew.rs/) to render the front-end because it's easy enough to setup for a Rust project. Maybe sometime in the future it'll move to something that is mostly server-rendered because that's better...

The back-end is built on [actix-web](https://actix.rs/), but it's basically just a proxy for the CRF2 API with some goodies (e.g. RC2 authentication and serving the front-end files). The CRF2 API is implemented in [libfj](https://github.com/NGnius/libfj) for reusability... and because I already had most FJ APIs in that project.
