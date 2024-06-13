use source::Source;
use tokenizer::generate_tokens;
use traits::error_display::ErrorDisplay;

mod source;
mod tokenizer;
mod traits;

pub fn convert(input: String) -> Result<(), ()> {
    let source = Source::from(input);

    let tokens = match generate_tokens(source.clone()) {
        Ok(tokens) => tokens,
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err.display(source.clone()));
            }
            return Err(());
        }
    };

    dbg!(tokens);

    Ok(())
}
