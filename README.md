# Ruget

[![Build Status](https://travis-ci.org/KeisukeToyota/ruget.svg?branch=master)](https://travis-ci.org/KeisukeToyota/ruget)

Alternative to wget written in Rust

![screen shot](https://github.com/KeisukeToyota/ruget/blob/images/screen_shot.png)

## install

```bash
$ git clone https://github.com/KeisukeToyota/ruget
$ cd ruget
$ cargo install --path .
```
In macOS you can install it with Homebrew
```bash
$ brew tap keisuketoyota/homebrew-ruget
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