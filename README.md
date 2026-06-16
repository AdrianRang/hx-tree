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

---
No AI used in the development of this TUI

---

## Roadmap
- [X] Add file and folder renaming.
  - [ ] Keep expanded folders expanded on rename
- [ ] Open both helix and hx-tree with `hxt path`.
- [ ] give tmux session a name to avoid conflicts with other apps and allow for multiple sessions.
- [X] Add file and folder icons
  - [X] add all icons in `icon_util.rs`
  - [ ] Add color to icons
- [X] Reload button
- [ ] Load children dinamically instead of all at once
- [ ] Add color to icons
- [ ] Add cursor support
- [X] Arrow keys on text input
- [ ] Show controls on ui
- [ ] Styles
- [ ] Change to use std::fs (see note 1)
- [ ] Compress multiple nested files to single line

#### Note 1
I didn't use `std::fs` because I wanted more of a challenge and prectice using bash, but I've had enough and want to rewrite everithing using a more sane approach
