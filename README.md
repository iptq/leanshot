leanshot
========

[![](https://api.travis-ci.org/iptq/leanshot.svg?branch=develop)](https://travis-ci.org/iptq/leanshot)

Screenshot-capturing utility.

Requirements
------------

You must have imlib2 and OpenGL installed. Fortunately, these are relatively common libraries.

Installation
------------

Binary distributions are available on the [releases](https://github.com/iptq/leanshot/releases) page.

To install from crates.io, use:

```
cargo install leanshot
```

Example Integration
-------------------

It's nice to have a script like:

```bash
#!/bin/bash
SCREENSHOT=$HOME/.cargo/bin/leanshot
# choose some file to save it to
FILE="/path/to/screenshot.png"
$SCREENSHOT $1 -o $FILE
# optional: copy to clipboard
XCLIP=/usr/bin/xclip
$XCLIP -selection clipboard -t image/png -i $FILE
```

Then, you can bind this script to the keybinds of your choice using your window manager's config.

Contact
-------

Author: Michael Zhang

License: MIT
