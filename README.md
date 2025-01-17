# Coreutils in Rust

[![Hits-of-Code](https://hitsofcode.com/github/GrayJack/coreutils)](https://hitsofcode.com/view/github/GrayJack/coreutils)
[![Build Status](https://api.travis-ci.com/GrayJack/coreutils.svg?branch=master)](https://travis-ci.com/GrayJack/coreutils)
[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FGrayJack%2Fcoreutils.svg?type=shield)](https://app.fossa.io/projects/git%2Bgithub.com%2FGrayJack%2Fcoreutils?ref=badge_shield)


An attempt to make safe coreutils aiming a minimal and yet complete set of utilities, adding only features that are common between several implementations of the utility and functions that are really useful.

This project has no intent to be 100% compatible with GNU's coreutils, like
[Uutils' coreutils](https://github.com/uutils/coreutils)

## Contributing
Check the [CONTRIBUTING.md](./CONTRIBUTING.md) file for the guidelines to contribute to the project, including issue reports, git commits messages, etc.

## Minimum Rust Version Policy
This project's minimum supported `rustc` version (MSRV) is 1.37.0.

In general, this project will try to be conservative with respect to the minimum supported version of Rust, but in case of safety reasons it may bump at any time. [e.g. `MaybeUninit` stabilization on 1.36.0 fixing huge problems with `std::mem::uninitialized()`]

## Tools
|   Name   | Not Started | Started | Done |
|:--------:|:-----------:|:-------:|:----:|
| basename |             |         |   X  |
|    cat   |      X      |         |      |
|   chgrp  |      X      |         |      |
|   chmod  |      X      |         |      |
|   chown  |      X      |         |      |
|  chroot  |      X      |         |      |
|   comm   |      X      |         |      |
|    cp    |      X      |         |      |
|  csplit  |      X      |         |      |
|    cut   |      X      |         |      |
|   date   |      X      |         |      |
|    dd    |      X      |         |      |
|    df    |      X      |         |      |
|   diff   |      X      |         |      |
|  dirname |             |         |   X  |
|    du    |      X      |         |      |
|   echo   |             |         |   X  |
|    env   |      X      |         |      |
|  expand  |      X      |         |      |
|   expr   |      X      |         |      |
|   false  |             |         |   X  |
|  groups  |             |         |   X  |
|   hash   |      X      |         |      |
|   head   |      X      |         |      |
|    id    |             |    X    |      |
|  install |      X      |         |      |
|   join   |      X      |         |      |
|   link   |      X      |         |      |
|    ln    |      X      |         |      |
|  logname |             |         |   X  |
|    ls    |      X      |         |      |
|   mkdir  |      X      |         |      |
|  mkfifo  |      X      |         |      |
|    mv    |      X      |         |      |
|   nice   |      X      |         |      |
|    nl    |      X      |         |      |
|   nohup  |      X      |         |      |
|    od    |      X      |         |      |
|   paste  |      X      |         |      |
|   patch  |      X      |         |      |
|  printf  |      X      |         |      |
|    pwd   |             |         |   X  |
|    rm    |      X      |         |      |
|   rmdir  |      X      |         |      |
|    sed   |      X      |         |      |
|    seq   |      X      |         |      |
|   sort   |      X      |         |      |
|   sleep  |             |         |   X  |
|   split  |      X      |         |      |
|   stat   |      X      |         |      |
|   stty   |      X      |         |      |
|   tail   |      X      |         |      |
|    tee   |      X      |         |      |
|   test   |      X      |         |      |
|   time   |      X      |         |      |
|   touch  |      X      |         |      |
|    tr    |      X      |         |      |
|   true   |             |         |   X  |
|   tsort  |      X      |         |      |
|    tty   |             |         |   X  |
|   uname  |      X      |         |      |
| unexpand |      X      |         |      |
|   uniq   |      X      |         |      |
|  unlink  |      X      |         |      |
|  uptime  |      X      |         |      |
|   users  |      X      |         |      |
|    wc    |      X      |         |      |
|    who   |      X      |         |      |
|  whoami  |             |         |   X  |
|    yes   |             |         |   X  |


## Licensing
This software is licensed under the [Mozilla Public License, v. 2.0](./LICENSE). If a copy of the MPL was not distributed with this file, you can obtain one at http://mozilla.org/MPL/2.0/.


[![FOSSA Status](https://app.fossa.io/api/projects/git%2Bgithub.com%2FGrayJack%2Fcoreutils.svg?type=large)](https://app.fossa.io/projects/git%2Bgithub.com%2FGrayJack%2Fcoreutils?ref=badge_large)