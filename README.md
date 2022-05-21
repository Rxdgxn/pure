Got bored, so i decided to make some sort of transpiler from Pure (a language i invented with this occasion) to Lua (Love 2D to be precise). And yes, i know the code is messy.

A .pure file has 3 "entry points", for loading, updating, drawing, and an end segment. Check the example file for more details about the structure.
Any line that begins with '@' will be treated as Pure code, but anything else will just be copy-pasted, even if it's not valid Lua code.

The aim of this project is to simplify even more game-developing, by pretty much adding shortcuts to writing Lua code.
