# rust-wena-app

In this test I'm using wena + evalexpr to create an elegant starting point for your console application with sophisticated evaluator that solves equation

```shell
git clone this repository
cargo build

irá gerar o binário rust-wana-app.exe
rust-wana-app.exe eval "(32*5)/4.0"

# Também pode usar o cargo para depurar
cargo run -q -- eval "(32*5)/4.0"
```

Irá mostrar o seguinte resultado
![](Console.png)

Você pode criar vários comandos 

```Rust
        .commands([Command::new("sum")
            .description("sum two numbers")
            .definition([Argument::new("first").required(true)])
            .definition([Argument::new("second").required(true)])
            .handler(|app| {
                let first = app.input.argument::<i32>("first").unwrap();
                let second = app.input.argument::<i32>("second").unwrap();
                               
                app.output.write(Alert::info(format!("{}+{} = {}", first, second, first + second )));

                Ok(0)
            })])
```


