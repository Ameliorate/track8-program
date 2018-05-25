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
` ` NOP 

`#` Move down one program. The interpreter will exit if trying to move up or down to a nonexistant program. 
`^` Move up a program 
`0-9` Move to the program that corresponds to the digit. Program 10 will be selected if `0` 

`!` Pops a value from the stack. Pushes 1 if the value was 0, pushes 0 otherwise. 
`=` Pops two values from the stack. Pushes 1 if the values were equal, 0 otherwise.

`+` Pop two numbers from the stack, sum them, and then push the sum onto the stack. 
`-` Pop two numbers from the stack, subract them, and then push the sum onto the stack. 
`*` Multiply two numbers on the stack. 
`%` Divide two numbers on the stack, using integer division. 
`d` Pop a number from the stack and print it to stdout in decimal. 
`D` Pop a number from the stack and print it to stderr in decimal. 

`~` Duplicate the number on the top of the stack. 
`,` Pop a number from the stack and discard it. 

`:` Enter Program Mode 
`|` Enter Read Mode 
`]` Enter Write Mode 
`>` Enter Stack Push Mode 
`"` Enter Print Mode 

### Program Move
`0-9` Interpeted as a decimal number 
`.` Go back to Main Mode and move to the program as dictated by the previous number.

This mode is normally used as in the code fragment `:27.`. This would (in Main Mode) move to the program 27.
It only really makes sense though when more than 8 programs are allowed.

If the selected program number is not allowed in the interpreter, nothing will happen and Main Mode will be returned to on the same program,

#### Example
```
:77.
```
Goes to program number 77, if it is allowed in the interpreter.

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

Anything else is the text to be printed.

#### Example
```
"Hello World!"
"Error: Started on an unreachable program!`
```
Prints `Hello World!`. The error will not be printed.
