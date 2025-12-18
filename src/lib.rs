use heck::{AsShoutySnakeCase, AsUpperCamelCase};
use smol_str::{format_smolstr, SmolStr};

pub trait ToUpperCamelCase {
    fn to_upper_camel_case(&self) -> SmolStr;
}

impl ToUpperCamelCase for str {
    fn to_upper_camel_case(&self) -> SmolStr {
        format_smolstr!("{}", AsUpperCamelCase(self))
    }
}

pub trait ToPascalCase {
    fn to_pascal_case(&self) -> SmolStr;
}

impl<TUpperCamel: ?Sized + ToUpperCamelCase> ToPascalCase for TUpperCamel {
    fn to_pascal_case(&self) -> SmolStr {
        self.to_upper_camel_case()
    }
}

pub trait ToShoutySnakeCase {
    fn to_shouty_snake_case(&self) -> SmolStr;
}

impl ToShoutySnakeCase for str {
    fn to_shouty_snake_case(&self) -> SmolStr {
        format_smolstr!("{}", AsShoutySnakeCase(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_upper_camel_case() {
        assert_eq!("foo_bar".to_upper_camel_case(), SmolStr::from("FooBar"));
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!("foo_bar".to_pascal_case(), SmolStr::from("FooBar"));
    }

    #[test]
    fn test_to_shouty_snake_case() {
        assert_eq!("fooBar".to_shouty_snake_case(), SmolStr::from("FOO_BAR"));
    }
}
