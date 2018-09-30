leanshot
========

Screenshot-capturing utility.

Installation
------------

So far, only installation from crates.io is supported:

```
cargo install leanshot
```

Binary distributions will soon be available on the releases page.

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
