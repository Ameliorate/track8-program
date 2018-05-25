# 8track-program
Esolang written upon an [8 track tape](https://en.wikipedia.org/wiki/8-track_tape).
Programs either loop implicitly or through the use of a metal strip on the tape, move down one program.

A program is written using a number of one-character codes, and a cartridge can contain up to 8 programs.
Each program can switch between programs on the cartridge, in addition to being able to modify the contents of each tape,
run different operations, and manipulate a 8-element stack.

This implementation also provides "cheats" to allow cartridges of more than 8 programs and with a stack larger than 8 elements.

# Coding Style
This project also experiments with using an APL-like programming style in rust.
For an example of this programming style, see the [J Incunabulum](http://code.jsoftware.com/wiki/Essays/Incunabulum).
