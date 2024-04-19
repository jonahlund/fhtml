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

impl fmt::Display for entity::Segment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            entity::Segment::Value => {
                write!(f, "{{}}")?;
            }
            entity::Segment::OpeningTag { name, attributes } => {
                write!(f, "<{name}")?;
                for attribute in attributes {
                    write!(f, " {attribute}")?;
                }
                write!(f, ">")?;
            }
            entity::Segment::ClosingTag { name } => {
                write!(f, "</{name}>")?;
            }
            entity::Segment::SelfClosingTag { name, attributes } => {
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
        for segment in &self.segments {
            write!(f, "{segment}")?;
        }

        Ok(())
    }
}
