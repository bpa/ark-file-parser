# Ark File Parser

Rust parsing library for Ark: Survival Evolved save files

# Motivation

I run a dedicated ark server on an older linux laptop with 8G of ram.  I'm also enjoying delving into more complex Rust projects to get more comfortable with how Rust does memory management.  

This started when a juvenile Pteranadon flew away never to be seen again and my kids asked if there was a way to find it.  Later, it was moving bases (before we got cryopods) and dinos fell off the boat somewhere along the way.  I have only been able to find windows based solutions for ark tools like [ArkBot](https://github.com/ark-mod/ArkBot).  

I used the [savegame toolkit](https://github.com/ark-mod/ArkSavegameToolkitNet) to find missing dinos.  Once started on the path of getting info from saves, its hard to stop.  The next question that came up was breeding stats for our tames.  You see where this is going.

# Build Status

[![Build Status](https://travis-ci.com/bpa/ark-file-parser.svg?branch=main)](https://travis-ci.com/bpa/ark-file-parser)

# Language

[Rust](https://www.rust-lang.org/)

# Features

Parses the map savefiles.  Current example main outputs a json of tames, a json of wild, and a json of eggs & gestating dinos.  On my laptop (Intel(R) Core(TM) i7-4510U CPU @ 2.00GHz), this takes about half a second for The Island and writes 6M of data.

# API

See [main.rs](src/main.rs) for example usage.

# Contributing

This project is mostly for fun and learning on my part. Any contributions are welcome. Feature requests and bugs are welcome. I will not leave pull requests hanging.

# Credits

[Savefile rundown](https://us-central.assets-static-2.romanport.com/ark/?v=2) that I found. If you know who created that, please let me know so I can properly attribute them.

[ark-mod's ARK Savegame Toolkit .NET Core](https://github.com/ark-mod/ArkSavegameToolkitNet)

# License

MIT