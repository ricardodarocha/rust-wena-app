use wena::*;

fn main() {
    Application::new("calculator")
        .commands([Command::new("eval")
            .description("Solve an expression")
            .definition([
                Argument::new("expression").required(true),
            ])
            .handler(|app| {
                let expression = app.input.argument::<String>("expression").unwrap();

                app.output.writeln(
                    Alert::info(
                        format!("{}\n{}", expression, expression)
                    )
                );

                Ok(0)
            })])
        .run();
}