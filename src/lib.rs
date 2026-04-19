//! Convert identifiers and string literals to different case styles
//! ## Motivation
//! I wanted a macro to convert identifiers to camel case string literals for use in a project, but I only found one that converted to pascal case. So, I forked it and added support for all the case styles you could ever want, so no one else has the same problem I had. I also added support for converting string literals, because why not?
//! ## Installation
//! Add it to your project with cargo:
//! ```
//! cargo add case_conv_macros
//! ```
//! ## Example
//! ```
//! use case_conv_macros::{identifier_to_camel, literal_to_sentence};
//!
//! let my_rusty_identifier = identifier_to_camel!(my_rusty_identifier);
//! assert_eq!(my_rusty_identifier, "myRustyIdentifier");
//! 
//! let my_rusty_literal = literal_to_sentence!("my_rusty_literal");
//! assert_eq!(my_rusty_literal, "My rusty literal");
//! ```

use inflector::Inflector;
use proc_macro::{Literal, TokenStream, TokenTree};

/// Convert an identifier to a camel case string literal
/// e.g. `my_rusty_identifier` becomes `"myRustyIdentifier"`
#[proc_macro]
pub fn identifier_to_camel(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string().to_camel_case();
                Literal::string(&ident).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert a string literal to a camel case literal
/// e.g. `"my_rusty_literal"` becomes `"myRustyLiteral"`
#[proc_macro]
pub fn literal_to_camel(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Literal(ref literal) => {
                let literal = literal.to_string().to_camel_case();
                Literal::string(&literal).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert an identifier to a kebab case string literal
/// e.g. `my_rusty_identifier` becomes `"my-rusty-identifier"`
#[proc_macro]
pub fn identifier_to_kebab(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string().to_kebab_case();
                Literal::string(&ident).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert a string literal to a kebab case literal
/// e.g. `"my_rusty_literal"` becomes `"my-rusty-literal"`
#[proc_macro]
pub fn literal_to_kebab(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Literal(ref literal) => {
                let literal = literal.to_string().to_kebab_case();
                Literal::string(&literal).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert an identifier to a train case string literal
/// e.g. `my_rusty_identifier` becomes `"My-Rusty-Identifier"`
#[proc_macro]
pub fn identifier_to_train(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string().to_train_case();
                Literal::string(&ident).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert a string literal to a train case literal
/// e.g. `"my_rusty_literal"` becomes `"My-Rusty-Literal"`
#[proc_macro]
pub fn literal_to_train(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Literal(ref literal) => {
                let literal = literal.to_string().to_train_case();
                Literal::string(&literal).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert an identifier to a screaming snake case string literal
/// e.g. `myRustyIdentifier` becomes `"MY_RUSTY_IDENTIFIER"`
#[proc_macro]
pub fn identifier_to_screaming_snake(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string().to_screaming_snake_case();
                Literal::string(&ident).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert a string literal to a screaming snake case literal
/// e.g. `"myRustyLiteral"` becomes `"MY_RUSTY_IDENTIFIER"`
#[proc_macro]
pub fn literal_to_screaming_snake(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Literal(ref literal) => {
                let literal = literal.to_string().to_screaming_snake_case();
                Literal::string(&literal).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert an identifier to a sentence case string literal
/// e.g. `my_rusty_identifier` becomes `"My rusty identifier"`
#[proc_macro]
pub fn identifier_to_sentence(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string().to_sentence_case();
                Literal::string(&ident).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert a string literal to a sentence case literal
/// e.g. `"my_rusty_literal"` becomes `"My rusty literal"`
#[proc_macro]
pub fn literal_to_sentence(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Literal(ref literal) => {
                let literal = literal.to_string().to_sentence_case();
                Literal::string(&literal).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert an identifier to a snake case string literal
/// e.g. `myRustyIdentifier` becomes `"my_rusty_identifier"`
#[proc_macro]
pub fn identifier_to_snake(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string().to_snake_case();
                Literal::string(&ident).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert a string literal to a snake case literal
/// e.g. `"myRustyLiteral"` becomes `"my_rusty_literal"`
#[proc_macro]
pub fn literal_to_snake(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Literal(ref literal) => {
                let literal = literal.to_string().to_snake_case();
                Literal::string(&literal).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert an identifier to a pascal case string literal
/// e.g. `my_rusty_identifier` becomes `"MyRustyIdentifier"`
#[proc_macro]
pub fn identifier_to_pascal(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string().to_pascal_case();
                Literal::string(&ident).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert a string literal to a pascal case literal
/// e.g. `"my_rusty_literal"` becomes `"MyRustyLiteral"`
#[proc_macro]
pub fn literal_to_pascal(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Literal(ref literal) => {
                let literal = literal.to_string().to_pascal_case();
                Literal::string(&literal).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert an identifier to a title case string literal
/// e.g. `my_identifier_is_rusty` becomes `"My Identifier Is Rusty"`
#[proc_macro]
pub fn identifier_to_title(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Ident(ref ident) => {
                let ident = ident.to_string().to_title_case();
                Literal::string(&ident).into()
            }
            _ => token_tree,
        })
        .collect()
}

/// Convert a string literal to a title case literal
/// e.g. `"my_literal_is_rusty"` becomes `"My Literal Is Rusty"`
#[proc_macro]
pub fn literal_to_title(stream: TokenStream) -> TokenStream {
    stream
        .into_iter()
        .map(|token_tree| match token_tree {
            TokenTree::Literal(ref literal) => {
                let literal = literal.to_string().to_title_case();
                Literal::string(&literal).into()
            }
            _ => token_tree,
        })
        .collect()
}