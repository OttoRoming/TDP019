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
  - [ ] lägga till "matching" method till Type för att kolla om två typer kan matcha, så som till exempel en tom list och en lista med ints matchar

## Källor
- https://www.ida.liu.se/~TDP019/
- https://interpreterbook.com/
- https://craftinginterpreters.com/
- https://www.w3schools.com/cpp/cpp_operators_precedence.asp
- https://en.wikipedia.org/wiki/Escape_sequences_in_C
- https://www.reddit.com/r/ProgrammingLanguages/comments/pemg55
