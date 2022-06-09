use evalexpr::*;
use wena::*;

fn main() {
    Application::new("calculator")
        .commands([Command::new("eval")
            .description("Solve an expression")
            .definition([Argument::new("expression").required(true)])
            .handler(|app| {
                let expression = app.input.argument::<String>("expression").unwrap();
                
                
                app.output.write(Alert::info(format!("{} = {}", expression, eval(&expression[..]).unwrap())));

                Ok(0)
            })])
        .run();
}
