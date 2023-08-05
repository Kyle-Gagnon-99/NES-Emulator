use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub enum Directive {
    Org(u16)
}