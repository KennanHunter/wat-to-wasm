use source::Source;
use tokenizer::generate_tokens;

mod source;
mod tokenizer;
mod traits;

pub fn convert(input: String) -> Result<(), ()> {
    let source = Source::from(input);

    let _tokens = match generate_tokens(source.clone()) {
        Ok(_) => todo!(),
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err.display(source.clone()));
            }
        }
    };

    Ok(())
}
