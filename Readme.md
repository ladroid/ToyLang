# Toy language

Example how to develop AST tree, parser and lexer

## How to use

```rust
fn main() {
    println!("Hello, world!");
    let lexer = Lexer::new("2 + 2 * 2");
    let mut parser = Parser::new(lexer);
    let ast = parser.expr();
    println!("AST: {:?}", ast);

    let mut interpreter = Interpreter {
        symbol_table: std::collections::HashMap::new(),
    };

    println!("Result: {}", interpreter.eval(&ast));

    println!("=============================");
    let lexer_if = Lexer::new("if 3 < 2 then 1+2 else 1*4");
    let mut parser_if = Parser::new(lexer_if);
    let ast_if = parser_if.if_then_else();
    println!("AST: {:?}", ast_if);

    let mut interpreter_if = Interpreter {
        symbol_table: std::collections::HashMap::new(),
    };

    println!("Result: {}", interpreter_if.eval(&ast_if));

    println!("=============================");
    let lexer_print = Lexer::new("print 2+2*2");
    let mut parser_print = Parser::new(lexer_print);
    let ast_print = parser_print.expr();
    println!("AST: {:?}", ast_print);

    let mut interpreter_print = Interpreter {
        symbol_table: std::collections::HashMap::new(),
    };

    println!("Result: {}", interpreter_print.eval(&ast_print));
}
```

## Features

**Completed**

:white_check_mark: Mathematical operations

:white_check_mark: If then else

:white_check_mark: Built-in function 

**Partial**

:question: While loop

**Will be**

:exclamation: Variable assign

:exclamation: While loop

:exclamation: Functions typed