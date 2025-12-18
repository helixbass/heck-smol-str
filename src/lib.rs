use heck::AsUpperCamelCase;
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
}
