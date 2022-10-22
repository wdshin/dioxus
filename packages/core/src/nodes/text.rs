use crate::{
    innerlude::{
        AttributeValue, ComponentPtr, Element, IntoAttributeValue, Properties, Scope, ScopeId,
        ScopeState, Template,
    },
    AnyEvent, Component, ElementId,
};
use bumpalo::{boxed::Box as BumpBox, Bump};
use std::{
    cell::{Cell, RefCell},
    fmt::{Arguments, Debug, Formatter},
};

/// A bump-allocated string slice and metadata.
pub struct VText<'src> {
    /// The [`ElementId`] of the VText.
    pub id: Cell<Option<ElementId>>,

    /// The text of the VText.
    pub text: &'src str,
}