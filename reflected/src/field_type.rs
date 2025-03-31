#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OptionalType {
    Float,
    Integer,
    Text,
    Date,
    Decimal,
    Bool,
    Duration,
    DateTime,
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
            Type::Duration => OptionalType::Duration,
            Type::DateTime => OptionalType::DateTime,
            Type::Optional(_) | Type::Enum => unreachable!(),
        }
    }

    pub const fn to_non_optional(&self) -> Type {
        match self {
            OptionalType::Float => Type::Float,
            OptionalType::Integer => Type::Integer,
            OptionalType::Text => Type::Text,
            OptionalType::Date => Type::Date,
            OptionalType::Decimal => Type::Decimal,
            OptionalType::Bool => Type::Bool,
            OptionalType::Duration => Type::Duration,
            OptionalType::DateTime => Type::DateTime,
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
    Enum,
    Duration,
    DateTime,
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
            if tp == opt.to_non_optional() {
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

    pub fn is_enum(&self) -> bool {
        self.is_type(Self::Enum)
    }

    pub fn is_duration(&self) -> bool {
        self.is_type(Self::Duration)
    }

    pub fn is_optional(&self) -> bool {
        matches!(self, Self::Optional(_))
    }

    pub fn get_optional(&self) -> Option<OptionalType> {
        match self {
            Self::Optional(op) => Some(*op),
            _ => None,
        }
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
