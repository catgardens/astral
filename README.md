# Astral

is a command line interface tool designed to interact with running Neovim instances, allowing users to list active instances and open files in a specified instance from different shells.

inspired by [neovim-remote][].

## usage

start a neovim instance:
```bash
nvim --listen /tmp/nvim/socket
```

then use astral in another shell:
```bash
# open a file
astral file
# the `open` command is inferred but can also be specified explicitly
astral --open file

# send keys
astral --send 'iabc<esc>'

# eval and expression
astral --expr 'bufname("")'
README.md

# by default astral will use `/tmp/nvim/socket` as socket
astral --remote /tmp/coolsocket --open file

# astral can also be used as a git editor with `--wait`
git config --global core.editor 'astral --wait'
# this command will wait till the file is closed
astral --wait
```

[neovim-remote]: https://github.com/mhinz/neovim-remote
