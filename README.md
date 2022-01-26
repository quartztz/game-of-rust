# conway's game of rust - an SDL2 implementation

[Conway's Game Of Life](en.wikipedia.org/wiki/Conway%27s_Game_Of_Life) is a cool mathematical game that is immensely enhanced by the ability to quickly simulate a game state using a computer. This explains its popularity as a code project, and my interest in tossing my hat in the ring when writing my own implementation using the [sdl2](https://docs.rs/sdl2/latest/sdl2/) bindings in rust, to learn the language.

## background

I've already implemented this a while ago using pancurses. While i do prefer the terminal aesthetic, this does have a lot more features: i'm keeping the repo open, but if i do eventually work on this project again, it's gonna be here.

## controls and customization

Colors can be customized at the top of `grid.rs`, defined as constants.

Size, scale, and fullscreen-itude are defined at the top of `main.rs`.

You can additionally change the "probability" of the cells being alive at the start of the game by changing `P` at the top of `grid.rs`. For reference, `1.0` means no alive cell, and `0.0` means no dead cell.

Default controls are: 

 - left mouse button: toggle cell
 - enter: toggle start/pause
 - esc: quit
 - plus/minus: speed up/slow down game
 - R [when paused/setup]: reload grid to random state
 - Q [when paused/setup]: reload grid to empty state, and to setup phase
 - I [when paused]: import file "gun_and_eater@20.txt" in the project's root dir
 - E [when paused]: export current state to file "export.txt"

## issues/todo

 - [ ] i thought i had fixed the edge issue by having it loop around: apparently not, but the issue is fixed and i don't know _exactly_ what it does but i'll leave it at that for now.

 - [x] allow reading states from an input file, maybe a list of binary strings? TEMPLATE: 1920x1080 @ 40px scale => 27 lines of 48 characters (potentially 0s and 1s)

 - [ ] allow choosing which file to import

 - [x] allow exporting current state to file in the same format as reader