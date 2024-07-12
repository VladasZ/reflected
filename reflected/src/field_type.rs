#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionalType {
    Float,
    Integer,
    Text,
    Date,
    Decimal,
    Bool,
}

impl OptionalType {
    pub const fn from_type(tp: &Type) -> Self {
        match tp {
            Type::Float => OptionalType::Float,
            Type::Integer => OptionalType::Integer,
            Type::Text => OptionalType::Text,
            Type::Date => OptionalType::Date,
            Type::Decimal => OptionalType::Decimal,
            Type::Bool => OptionalType::Bool,
            _ => unreachable!(),
        }
    }

    pub const fn to_type(&self) -> Type {
        match self {
            OptionalType::Float => Type::Float,
            OptionalType::Integer => Type::Integer,
            OptionalType::Text => Type::Text,
            OptionalType::Date => Type::Date,
            OptionalType::Decimal => Type::Decimal,
            OptionalType::Bool => Type::Bool,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Float,
    Integer,
    Text,
    Date,
    Decimal,
    Bool,
    Custom,
    Optional(OptionalType),
}

impl Type {
    pub const fn to_optional(self) -> Self {
        Self::Optional(OptionalType::from_type(&self))
    }

    pub fn is_type(&self, tp: Self) -> bool {
        if self == &tp {
            return true;
        }

        if let Self::Optional(opt) = self {
            if tp == opt.to_type() {
                return true;
            }
        }

        false
    }

    pub fn is_float(&self) -> bool {
        self.is_type(Self::Float)
    }

    pub fn is_integer(&self) -> bool {
        self.is_type(Self::Integer)
    }

    pub fn is_text(&self) -> bool {
        self.is_type(Self::Text)
    }

    pub fn is_date(&self) -> bool {
        self.is_type(Self::Date)
    }

    pub fn is_decimal(&self) -> bool {
        self.is_type(Self::Decimal)
    }

    pub fn is_bool(&self) -> bool {
        self.is_type(Self::Bool)
    }

    pub fn is_custom(&self) -> bool {
        self.is_type(Self::Custom)
    }

    pub fn is_optional(&self) -> bool {
        matches!(self, Self::Optional(_))
    }

    pub fn is_number(&self) -> bool {
        self.is_integer() || self.is_float()
    }
}

#[cfg(test)]
mod test {
    use crate::Type;

    #[test]
    fn test() {
        assert!(Type::Float.is_float());
        assert!(Type::Float.to_optional().is_float());
    }
}
