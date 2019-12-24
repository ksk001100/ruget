# Ruget

![](https://img.shields.io/github/stars/ksk001100/ruget.svg)
![](https://img.shields.io/github/release/ksk001100/ruget.svg)
![](https://img.shields.io/github/issues/ksk001100/ruget.svg)
![](https://img.shields.io/github/forks/ksk001100/ruget.svg)
![](https://img.shields.io/github/license/ksk001100/ruget.svg)
[![Build Status](https://travis-ci.org/ksk001100/ruget.svg?branch=master)](https://travis-ci.org/ksk001100/ruget)

Alternative to wget written in Rust

![screen shot](https://github.com/ksk001100/ruget/blob/images/screen_shot.png)

## install

```bash
$ cargo install ruget
```

In macOS you can install it with Homebrew
```bash
$ brew tap ksk001100/homebrew-ruget
$ brew install ruget
```

## usage

```bash
$ ruget https://sample-videos.com/img/Sample-png-image-30mb.png
```

## Ruget vs Wget

### Ruget
```bash
$ time ruget http://ipv4.download.thinkbroadband.com/100MB.zip
...
...
...

ruget http://ipv4.download.thinkbroadband.com/100MB.zip  2.00s user 3.38s system 33% cpu 15.858 total
```

### Wget
```bash
$ time wget http://ipv4.download.thinkbroadband.com/100MB.zip
...
...
...

wget http://ipv4.download.thinkbroadband.com/100MB.zip  0.34s user 1.84s system 8% cpu 26.428 total
```