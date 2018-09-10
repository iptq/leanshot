screenshot
==========

Screenshot-capturing utility.

Installation
------------

Check out the releases page.

Example Integration
-------------------

```bash
#!/bin/bash
SCREENSHOT=$HOME/.cargo/bin/screenshot
$SCREENSHOT $1 --clipboard --output "$HOME/Screenshots/Screenshot_%Y%m%d-%H:%M:%S.png"
```

Building from Source
--------------------

First, clone this repository. Then, run

```
cargo build
```

Happy hacking!

Contact
-------

Author: Michael Zhang

License: MIT
