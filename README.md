B2
==

[![Build Status](https://travis-ci.org/azdle/b2.svg?branch=master)](https://travis-ci.org/azdle/b2)
[![Crates.io Link](http://meritbadge.herokuapp.com/b2)](https://crates.io/crates/b2)

Set your pins to 'B2'.


Installing
---------------
The easiest way to use b2 is to install it through cargo.

```
$ cargo install b2
```


Usage
-----

```
B2 - Set your pins to B2

A tool to XOR images together.

Usage:
  b2 makenoise <file> <outfile>
  b2 xor <file> <file2> <outfile>
  b2 (-h | --help)
  b2 (-v | --version)

Options:
  -h --help     Show this screen.
  -v --version     Show version.
```

Example

Make a noise image and encrypt your original image with that noise:

```
$ b2 makenoise sonofa.jpg noise.png
$ b2 xor sonofa.jpg noise.png encrypted.png
```

You now have an 'encrypted' picture, `encrypted.png`. To 'decrypt', just xor the encrypted image with the noise again:

```
$ b2 xor noise.png encrypted.png decrypted.png
```
