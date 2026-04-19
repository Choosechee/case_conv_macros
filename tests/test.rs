use case_conv_macros::*;

#[test]
fn camel_case() {
    let my_rusty_identifier =
        identifier_to_camel!(my_rusty_identifier);
    assert_eq!(my_rusty_identifier, "myRustyIdentifier");

    let my_rusty_literal =
        literal_to_camel!("my_rusty_literal");
    assert_eq!(my_rusty_literal, "myRustyLiteral");
}

#[test]
fn kebab_case() {
    let my_rusty_identifier =
        identifier_to_kebab!(my_rusty_identifier);
    assert_eq!(my_rusty_identifier, "my-rusty-identifier");

    let my_rusty_literal =
        literal_to_kebab!("my_rusty_literal");
    assert_eq!(my_rusty_literal, "my-rusty-literal");
}

#[test]
fn train_case() {
    let my_rusty_identifier =
        identifier_to_train!(my_rusty_identifier);
    assert_eq!(my_rusty_identifier, "My-Rusty-Identifier");

    let my_rusty_literal =
        literal_to_train!("my_rusty_literal");
    assert_eq!(my_rusty_literal, "My-Rusty-Literal");
}

#[test]
#[allow(nonstandard_style)]
fn screaming_snake_case() {
    let myRustyIdentifier =
        identifier_to_screaming_snake!(myRustyIdentifier);
    assert_eq!(myRustyIdentifier, "MY_RUSTY_IDENTIFIER");

    let myRustyLiteral =
        literal_to_screaming_snake!("myRustyLiteral");
    assert_eq!(myRustyLiteral, "MY_RUSTY_LITERAL");
}

#[test]
fn sentence_case() {
    let my_rusty_identifier =
        identifier_to_sentence!(my_rusty_identifier);
    assert_eq!(my_rusty_identifier, "My rusty identifier");

    let my_rusty_literal =
        literal_to_sentence!("my_rusty_literal");
    assert_eq!(my_rusty_literal, "My rusty literal");
}

#[test]
#[allow(nonstandard_style)]
fn snake_case() {
    let myRustyIdentifier =
        identifier_to_snake!(myRustyIdentifier);
    assert_eq!(myRustyIdentifier, "my_rusty_identifier");

    let myRustyLiteral =
        literal_to_snake!("myRustyLiteral");
    assert_eq!(myRustyLiteral, "my_rusty_literal");
}

#[test]
fn pascal_case() {
    let my_rusty_identifier =
        identifier_to_pascal!(my_rusty_identifier);
    assert_eq!(my_rusty_identifier, "MyRustyIdentifier");

    let my_rusty_literal =
        literal_to_pascal!("my_rusty_literal");
    assert_eq!(my_rusty_literal, "MyRustyLiteral");
}

#[test]
fn title_case() {
    let my_identifier_is_rusty =
        identifier_to_title!(my_identifier_is_rusty);
    assert_eq!(my_identifier_is_rusty, "My Identifier Is Rusty");

    let my_literal_is_rusty =
        literal_to_title!("my_literal_is_rusty");
    assert_eq!(my_literal_is_rusty, "My Literal Is Rusty");
}