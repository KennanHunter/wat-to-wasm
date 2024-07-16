use source::Source;
use tokenizer::generate_tokens;

mod parser;
mod shared;
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

    let tree = match parser::parse_tokens(tokens) {
        Ok(tree) => tree,
        Err(err) => {
            eprintln!("{}", err.display(source));

            return Err(());
        }
    };

    dbg!(tree);

    Ok(())
}
