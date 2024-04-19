use std::fmt;

use crate::entity;

impl fmt::Display for entity::DashIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value.fmt(f)
    }
}

impl fmt::Display for entity::Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { name } = self;

        write!(f, "{name}=\"{{}}\"")
    }
}

impl fmt::Display for entity::Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            entity::Part::Value => {
                write!(f, "{{}}")?;
            }
            entity::Part::OpeningTag { name, attributes } => {
                write!(f, "<{name}")?;
                for attribute in attributes {
                    write!(f, " {attribute}")?;
                }
                write!(f, ">")?;
            }
            entity::Part::ClosingTag { name } => {
                write!(f, "</{name}>")?;
            }
            entity::Part::SelfClosingTag { name, attributes } => {
                write!(f, "<{name}")?;
                for attribute in attributes {
                    write!(f, " {attribute}")?;
                }
                write!(f, "/>")?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for entity::Html {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for part in &self.parts {
            write!(f, "{part}")?;
        }

        Ok(())
    }
}
