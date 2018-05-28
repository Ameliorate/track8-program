# 8track-program Specification

## File Format
An 8 track cartridge can be encoded as a utf-8 .8trk file as follows: 
Text inside curly braces is to be replaced with data. 
```
[{interpreter pragmas}]
{program 1}
{program 2}
{program 3}
{program 4}
{program 5}
{program 6}
{program 7}
{program 8}
```
If a program is shorter than the longest program, the program shall be extended with spaces.

A program is an array of UTF-8 characters, where each character will be used as an instruction.

The pragma may be omitted if there is no pragma.

## Program Model
The tape has 8 programs. Each program is a stream of characters that may also be interpreted as 8 bit numbers.
Each end of the tape is attached to each other, such that reaching the end of the program will move to the beginning of the same program.

The interpeter simulates a tape head. This tape head can access any value on any of the 8 programs it is over, and move right.
The tape head is unable to move leftwards or rewind the tape, because of the physical nature of the tape.

A 8 element stack is proveded for the workings of operators. If you try to push to a full stack, the pushed values will be discarded silently.

## Instructions
### Main Mode
#### NOP
` `

Does nothing.

#### Up/Down Program
`#` Down 
`^` Up
`/` Up If True
`\` Down If True

Moves up/down a program.

Up If True and Down If True will pop a value from the stack, and if that number is not equal to 0 move up or down accordingly.
If the number is equal to 0 nothing will happen.

The interpreter will exit if trying to move up or down to a nonexistant program. 

When moving up/down a program, the tape will be advanced a space, as normal.

#### Logical
`!` Not

Pops a value from the stack. Pushes 1 if the value was 0, pushes 0 otherwise. 

`=` Equals

Pops two values from the stack. Pushes 1 if the values were equal, 0 otherwise.

#### Math
`+` Add 
`-` Subtract 
`*` Multiply
`%` Divide

Pop two numbers from the stack, preform the desired operation upon them, and then push the result back onto the stack.

Division will use trunctuating integer division.

#### Print
`d` Stdout  
`D` Stderr 

Pop a number from the stack and print it to stdout/stderr in decimal. 

#### Stack
`~` Dup
`,` Discard

Dup will pop a value from the stack and push two copies of the value.

Discard will pop a value from the stack and discard it.

#### Modes
`|` Read Mode 
`]` Write Mode 
`>` Stack Push Mode 
`"` Print Mode 

Enters the assorted modes as described in the operator names.

### Read/Write Mode
`0-9` Interpreted as a decimal number 
`.` Go back to Main Mode and push/pop the value currently avaliable from the selected program to the stack 

The position of the period is where the value will be written on the selected program.

#### Example
```
>40.]3.#
    |3.d
```
Pushes decimal 40 to the stack, writes it to program 3, then reads it back out and prints it.

### Stack Push Mode
`0-9` Interpeted as a decimal number. 
`.` Goes back to Main Mode and pushes the previous number to the stack.

#### Example
```
>30.
```
Pushes decimal 30 to the stack.

### Print Mode
`"` Print the text to stdout and return to Main Mode. 
`\`` Print the text to stderr and return to Main Mode.
`\\` Escape character.

Anything else is the text to be printed.

#### Escape Characters
`\\\\` Literal backslash
`\n` Newline
`\"` Literal Quote
`\g` Literal Grave (replace g with a grave character)

#### Example
```
"Hello World!"
"Error: Started on an unreachable program!`
```
Prints `Hello World!`. The error will not be printed.
