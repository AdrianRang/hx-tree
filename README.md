# hx-tree

Tree file viewer for helix.

## Instalation
TODO

## How to use
hx-tree uses tmux to interact with helix so helix shoud be run inside tmux
`tmux hx .` in the project directory
then in a separate terminal run `hx-tree` in the project directory

Use up arrow | k to move up
Use down arrow | j to move down
Use right arrow | Space to expand a directrory or open a file
Use n to create a new file

## Roadmap
- [ ] Add file and folder renaming.
- [ ] Open both helix and hx-tree with `hxt path`.
- [ ] give tmux session a name to avoid conflicts with other apps.
- [ ] Add file and folder icons
- [ ] Reload button
- [ ] Load children dinamically instead of all at once
