<p align="center"><img src="https://github.com/phaazon/luminance-rs/blob/master/docs/imgs/luminance.svg" alt="luminance logo"/></p>
<h1 align="right"><b>luminance</b>, the elegant, safe, type-safe, stateless <i>and simple</i> graphics crate</h1>

[![Talk on IRC!](https://img.shields.io/badge/IRC-%23luminance%40irc.freenode.net-blueviolet?logo=wechat)](https://webchat.freenode.net)
[![Build Status](https://img.shields.io/travis/phaazon/luminance-rs?logo=travis)](https://travis-ci.org/phaazon/luminance-rs)
[![crates.io](https://img.shields.io/crates/v/luminance.svg?logo=rust)](https://crates.io/crates/luminance)
[![docs.rs](https://docs.rs/luminance/badge.svg)](https://docs.rs/luminance/)
![License](https://img.shields.io/crates/l/luminance)


<!-- vim-markdown-toc GFM -->

* [History](#history)
* [The luminance ecosystem](#the-luminance-ecosystem)
  * [Core crates](#core-crates)
  * [Backend crates](#backend-crates)
  * [Platform crates](#platform-crates)
  * [Other crates](#other-crates)
* [Learning](#learning)
* [Contributing](#contributing)
* [Dependent projects](#dependent-projects)
* [Licenses](#licenses)

<!-- vim-markdown-toc -->

# History

**luminance** is an effort to make graphics rendering simple and elegant. It was originally imagined,
designed and implemented by [@phaazon](https://github.com/phaazon) in Haskell ([here](https://hackage.haskell.org/package/luminance))
and eventually ported to Rust in 2016. The core concepts remained the same and the crate has been
slowly evolving ever since. At first, used only by @phaazon for his Rust demoscene productions (
example [here](https://github.com/phaazon/celeri-remoulade) and
[here](https://github.com/phaazon/outline-2017-invitro), using
[spectra](https://crates.io/crates/spectra)) and a bunch of curious peeps, it now has more visibility
among the graphics ecosystem of Rust.

Currently, such an ecosystem is spread into several crates, ideas and people. It is highly
recommended to read the great article about the ecosystem by @Icefoxen, [here](https://wiki.alopex.li/AGuideToRustGraphicsLibraries2019).

However, **luminance** is a bit different from what it was initially imagined for. People are
looking for an easy-to-use crate, with good abstractions and safe-guards against all the _bad_ and
_dangerous_ graphics API caveats. **luminance** has always been about providing a safe, type-safe
and elegant API (being Haskell-based makes it heavily use the type system, for instance) but it has
now a more accurate place in the ecosystem. Where [gfx-hal] provides you with an experience focused
on down-to-metal performance and an API very similar to [Vulkan]’s, **luminance** provides an API
that is, for sure, a bit less low-level — and hence, yes, it’s likely you will not have the same
performances as with [gfx-hal] (even though no benchmarks have been done so far), and the API is not
[Vulkan]-based — but easier to start with, especially if you don’t already have a background
experience with [OpenGL] or [Vulkan].

The strengths of **luminance** are:

- Easy to learn: the concepts, based on [OpenGL], are applied to _graphics_, not _general-purpose
  programming on GPU_. Using **luminance** will help you wrap your fingers around what graphics
  programming is about and it will help you to, perhaps, jump to lower abstractions like
  [gfx-hal], if you ever need to.
- Performant: by using Rust and being designed around the concept of good performances,
  **luminance** should allow you to build nice and fast simulations, animations and video games.
  Remember that games you played years ago didn’t have [Vulkan] and were impressive nonetheless.
  It’s unlikely you will get 100% out of your GPU by using **luminance** since it’s built over
  technologies that are not using 100% of your GPU. Unless you need and know exactly why you need
  100% of your GPU, you should be _just fine™_.
- Elegant: the design is heavily based on functional programming concepts such as typeclasses,
  associated types, singleton types, existentials, contravariant resources, procedural macros,
  strong typing, etc. Plus, every bit of possible _stateful_ computations is hidden behind a
  system of smart state, removing the need to worry about side-effects. **luminance** still has
  mutation (unless its Haskell version) but the Rust type system and borrow checker allows for
  safe mutations.
- Modern: the whole **luminance** ecosystem tries its best to stay up-to-date with Rust evolutions
  and features. On the same level, the underneath technologies are kept up-to-date and might even
  change if a more modern and more adapted one emerges ([Vulkan] might eventually get adopted but
  this is just an idea for now).
- _Enough opinionated_: a big bet with **luminance** was to make it opinionated, but not too much.
  It needs to be opinionated to allow some design constructs to be possible and optimize
  performance and allow for extra safety. However, it must not be too much to prevent it to become
  a _framework_. **luminance** is a _library_, not a _framework_, meaning that it will adapt to
  how **you** think you should design your software, not the other way around. That is limited to
  the design of **luminance** but you shouldn’t feel too hands-tied.

# The luminance ecosystem

It is currently composed of several different crates:

## Core crates

- [luminance]: the core crate, exposing a graphics API that aims to be easy to learn, safe,
  type-safe, stateless and fun!
- [luminance-derive]: a companion crate to [luminance] you’re very likely to enjoy; it will help
  you derive important traits for your application or library to work. You should definitely
  invest some time in the documentation of this crate; it’s easy and well explained.
- [luminance-front]: a _front facing_ set of [luminance] re-exports to make it easy to use the
  library as a end-user developer by picking a backend type at compile-time, most of the time
  based on your compilation target.

## Backend crates

- [luminance-gl]: a crate gathering OpenGL backends. Several versions might be supported.
- [luminance-webgl]: a crate gathering WebGL backends. Several versions might be supported.

## Platform crates

- [luminance-glfw]: a platform implementation for [GLFW](https://www.glfw.org)
  (via [glfw](https://crates.io/crates/glfw)).
- [luminance-glutin]: a platform implementation for [glutin].
- [luminance-sdl2]: a platform implementation for [sdl2].
- [luminance-web-sys]: a platform implementation for [web-sys].
- [luminance-windowing]: a small interface crate for windowing purposes. It’s unlikely you will
  need it, but it provides some basic and shared data structures you might use.

## Other crates

- [luminance-examples]: a combination of examples to show off some features / techniques.
- [luminance-examples-web]: same as above, but for the Web.

# Learning

[luminance] has two main and official resources to learn:

- The [book](https://rust-tutorials.github.io/learn-luminance). It contains various chapters,
  including tutorials and onboarding newcomers. It will not provide you with the best description
  of a given feature as it focuses more on the overall comprehension and explaining than code
  directly. It also fits people who don’t know anything about rendering.
- The [examples](luminance-examples/README.md). They are like unit tests: each introduces and
  focuses on a very specific aspect or feature. You should read them if you are interested in
  a specific feature. They’re not well suited to learn from scratch and they are weaker than a
  structured tutorial but more concise.

You should try both ways and see which one fits the best for you!

# Contributing

Please read the [CONTRIBUTING](CONTRIBUTING.md) document.

# Dependent projects

Those projects use luminance:

- [Céleri Rémoulade](https://github.com/phaazon/celeri-remoulade).
  - A demoscene production by [@phaazon](https://github.com/phaazon), released at Evoke 2016 in the
    PC Demo category.
- [Outline 2017 Invitro](https://github.com/phaazon/outline-2017-invitro).
  - A demoscene production by [@phaazon](https://github.com/phaazon),
  released at Revision 2017 in the PC Demo category.
- [Dali Renderer](https://github.com/austinjones/dali-rs)
  - A rendering library by [@austinjones](https://github.com/austinjones), designed to generate
    high-resolution digital paintings to be printed on canvas.
- [Rx](https://rx.cloudhead.io)
  - A modern and minimalist pixel editor. Rx's GL backend is built on luminance.
- [luminance-glyph](https://github.com/JohnDoneth/luminance-glyph)
  - A fast text renderer for luminance by [@JohnDoneth](https://github.com/JohnDoneth), powered by
    [glyph_brush](https://crates.io/crates/glyph_brush).

# Licenses

[luminance] is licensed under [BSD-3-Clause] and the logo is under [CC BY-ND].

[luminance]: ./luminance
[luminance-derive]: ./luminance-derive
[luminance-gl]: ./luminance-gl
[luminance-glfw]: ./luminance-glfw
[luminance-glutin]: ./luminance-glutin
[luminance-sdl2]: ./luminance-sdl2
[luminance-webgl]: ./luminance-webgl
[luminance-web-sys]: ./luminance-web-sys
[luminance-windowing]: ./luminance-windowing
[luminance-front]: ./luminance-front
[luminance-examples]: ./luminance-examples
[luminance-examples-web]: ./luminance-examples-web
[glutin]: https://crates.io/crates/glutin
[gfx-hal]: https://crates.io/crates/gfx-hal
[sdl2]: https://crates.io/crates/sdl2
[web-sys]: https://crates.io/crates/web-sys
[Vulkan]: https://www.khronos.org/vulkan
[Opengl]: https://www.khronos.org/opengl
[BSD-3-Clause]: https://opensource.org/licenses/BSD-3-Clause
[CC BY-ND]: https://creativecommons.org/licenses/by-nd/4.0
