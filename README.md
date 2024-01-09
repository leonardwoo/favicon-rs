# Favicon

[!["License"](https://img.shields.io/github/license/leonardwoo/favicon-rs?style=flat-square)](https://github.com/leonardwoo/favicon-rs/blob/main/LICENSE)
[!["Crates.io"](https://img.shields.io/crates/d/favicon-rs?style=flat-square)](https://srl.cx/jIUoJvBB)
[!["Crates.io"](https://img.shields.io/crates/v/favicon-rs?style=flat-square)](https://srl.cx/jIUoJvBB)

## Introduction

A favicon image generation tool.

## Install

```shell
cargo install favicon-rs
```

## Usage

```shell
favicon -i <image.png> [output_path]
```

## HTML Favicon

```html
<head>

<link rel="shortcut icon" type="image/x-icon" href="/favicon.ico" />
<!-- OR -->
<link rel="icon" type="image/x-icon" href="/favicon.ico" />

<link rel="icon" type="image/png" sizes="32x32" href="favicon-32x32.png" />
<link rel="icon" type="image/png" sizes="192x192" href="android-chrome-192x192.png" />
<link rel="icon" type="image/png" sizes="512x512" href="android-chrome-512x512.png" />
<link rel="apple-touch-icon" sizes="180x180" href="apple-touch-icon.png" />

</head>
```
