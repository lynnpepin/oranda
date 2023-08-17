<div class="oranda-hide">

# oranda

</div>

> 🎁 generate beautiful landing pages for your projects

[![crates.io](https://img.shields.io/crates/v/oranda.svg)](https://crates.io/crates/oranda)
[![CI](https://github.com/axodotdev/oranda/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/axodotdev/oranda/actions/workflows/ci.yml)
[![release](https://github.com/axodotdev/oranda/actions/workflows/release.yml/badge.svg)](https://github.com/axodotdev/oranda/actions/workflows/release.yml)
[![web](https://github.com/axodotdev/oranda/actions/workflows/web.yml/badge.svg?branch=main)](https://github.com/axodotdev/oranda/actions/workflows/web.yml)


`oranda` is an opinionated static-site generator that is designed for developers
who are publishing projects and would like a website but don't want to build
one from scratch.

<div class="oranda-hide">

`oranda` uses `oranda` so you can checkout a live example [here][website]!

## Installation

To install `oranda`, please visit the [`oranda` website][website]- which is generated by
`oranda`!

[website]: https://axodotdev.github.io/oranda

</div>

## Quickstart

```sh
# build your site
> oranda build

# build your site and start a server that rebuilds on file changes
> oranda dev
```

Here's an animated demo:

![oranda demo gif](https://github.com/axodotdev/oranda/assets/6445316/439082a6-2caa-477e-93cc-1ff985d9bb21)

## Configuration

First of all: `oranda` is designed to work without a configuration file. For a lot of projects,
you can likely just run `oranda build` and get a site that contains a couple of things that
`oranda` was automatically able to glean about your project. That being said, it also supports
a configuration file that allows you to tweak many things about oranda's behaviour.

If you'd like to configure `oranda`, place an `oranda.json` file in the root of
your project and fill it with the configuration you'd like. Check out the [docs]
to learn more about your configuration options!

[docs]: https://opensource.axo.dev/oranda/book/configuration.html

## Installers: integrating with `cargo-dist`

`oranda` has first-class integration with [`cargo-dist`], a tool that builds
distributable artifacts for your Rust applications. If you have `cargo-dist`
configured in your project correctly, `oranda` will be able to automatically
tell. Benefits of integrating with `cargo-dist` include:

- Installer scripts: `cargo-dist` can generate one-line installer scripts, which
  `oranda` will display in your generated page
- Guaranteed platform support: `oranda` tries to support as many platforms as it can,
  but if you build something with `cargo-dist`, we guarantee it'll show up correctly

[`cargo-dist`]: https://github.com/axodotdev/cargo-dist

## Contributing

Feel free to open a new issue or pull request if you notice something off or have a new feature
request! We sometimes tag issues with [good first issue] for issues that we think would make
a good learning experience for new contributors.

For local development on oranda, we also have a [special docs page][contributing-docs] with some tips.

[good first issue]: https://github.com/axodotdev/oranda/labels/good%20first%20issue
[contributing-docs]: https://opensource.axo.dev/oranda/book/contributing.html
