use crate::parser::errors::{
    ExpectedIdentifierError, ExpectedMethodError, ExpectedStringError, ExpectedTokenError,
    ExpectedTypeError,
};

macro_rules! define_boxable_conversion {
    [ $( $class:ident ),+ ] => {
        $(
            impl From<$class> for Box<dyn crate::traits::error_display::ErrorDisplay> {
                fn from(e: $class) -> Self {
                    Box::new(e)
                }
            }
        )+
    };
}

define_boxable_conversion![
    ExpectedIdentifierError,
    ExpectedStringError,
    ExpectedTypeError,
    ExpectedMethodError,
    ExpectedTokenError
];
