use source::Source;
use tokenizer::generate_tokens;

mod source;
mod tokenizer;
mod traits;

/// Take in the WAT source code and returns (TODO)
///
/// # Arguments
///
/// * `input` - String containing source code
///
/// # Examples
///
/// ```
/// use wat_to_wasm::convert;
///
/// convert("(module)".to_owned());
/// ```
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
