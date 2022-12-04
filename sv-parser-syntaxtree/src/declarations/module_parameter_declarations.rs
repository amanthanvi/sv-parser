use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node)]
pub enum LocalParameterDeclaration {
    Param(Box<LocalParameterDeclarationParam>),
    Type(Box<LocalParameterDeclarationType>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct LocalParameterDeclarationParam {
    pub nodes: (Keyword, DataTypeOrImplicit, ListOfParamAssignments),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct LocalParameterDeclarationType {
    pub nodes: (Keyword, Keyword, ListOfTypeAssignments),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub enum ParameterDeclaration {
    Param(Box<ParameterDeclarationParam>),
    Type(Box<ParameterDeclarationType>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct ParameterDeclarationParam {
    pub nodes: (Keyword, DataTypeOrImplicit, ListOfParamAssignments),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct ParameterDeclarationType {
    pub nodes: (Keyword, Keyword, ListOfTypeAssignments),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct SpecparamDeclaration {
    pub nodes: (
        Keyword,
        Option<PackedDimension>,
        ListOfSpecparamAssignments,
        Symbol,
    ),
}
