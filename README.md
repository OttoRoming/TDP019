# TDP019 - Oeno language interpreter
Implemented in 100% safe Rust 🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀🦀
@sermuns

## TODO
- [x] ta bort null från språket
    - [x] ta bort testfall relaterade till null
    - [x] ta bort null från lexer
    - [x] ta bort null från parser
    - [x] ta bort null från ast
    - [x] ändra i type checker för expressions
- [x] färdigställa parser
    - [x] block
    - [x] if
    - [x] while
    - [x] each
    - [x] var
    - [x] fun
    - [x] return
    - [x] expressions
        - [x] assign
        - [x] update
        - [x] binary
        - [x] unary
        - [x] identifier
        - [x] call
        - [x] index
        - [x] string
        - [x] int
        - [x] float
        - [x] bool
        - [x] list
  - [x] lägga till "matching" method till Type för att kolla om två typer kan matcha, så som till exempel en tom list och en lista med ints matchar
  - [x] förbjuda == mellan funktioner
  - [x] avskaffa variantly craten eftersom så jag slipper *value.bool_ref().unwrap()
  - [x] varna mot var l = [] för att det skapar en variabel med ambigous type
  - [x] referenser fungerar inte
  - [ ] kommentarer måste vara i början av en rad

måste fixa display för typer så att man inte får dessa felmedelanden
```
Check error: variable declaration type mismatch (specified: List(Some(Int)); got: Int)
  --> 1:1 -> 1:25
1 | var nums: List<Int> = 10
```

## Källor
- https://www.ida.liu.se/~TDP019/
- https://interpreterbook.com/
- https://craftinginterpreters.com/
- https://www.w3schools.com/cpp/cpp_operators_precedence.asp
- https://en.wikipedia.org/wiki/Escape_sequences_in_C
- https://www.reddit.com/r/ProgrammingLanguages/comments/pemg55
- https://youtu.be/HwupNf9iCJk
