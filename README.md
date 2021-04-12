# FPVSetup
[![Crates.io](https://img.shields.io/crates/v/fpvsetup)](https://crates.io/crates/fpvsetup "FPVSetup on Crates.io")
[![Docs.rs](https://img.shields.io/badge/documentation-docs.rs-informational)](https://docs.rs/fpvsetup "FPVSetup on Docs.rs")
[![Build Status](https://github.com/kotauskas/fpvsetup/workflows/Checks%20and%20tests/badge.svg)](https://github.com/kotauskas/fpvsetup/actions "GitHub Actions page for FPVSetup")

Library and GUI tool for calculating optimal first-person 3D view parameters from monitor size and distance.

The "portal-like" mode will perform the necessary trigonometry and output the configuration for the camera which will allow the screen to appear as if it is a portal into the rendered 3D world, improving perception of depth and realism.

The "focused" mode will output an FOV for the camera which will represent an accurate scale of objects at a given distance in the 3D world from the camera.

## License
The code itself is dual-licensed under the MIT or Apache 2.0 licenses, at your option.

The icon is the eye emoji from [Twemoji], licensed under [CC-BY 4.0].

[Twemoji]: https://twemoji.twitter.com/ " "
[CC-BY 4.0]: https://creativecommons.org/licenses/by/4.0/ " "
