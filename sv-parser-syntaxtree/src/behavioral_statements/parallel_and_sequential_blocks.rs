use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node)]
pub enum ActionBlock {
    StatementOrNull(Box<StatementOrNull>),
    Else(Box<ActionBlockElse>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct ActionBlockElse {
    pub nodes: (Option<Statement>, Keyword, StatementOrNull),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct SeqBlock {
    pub nodes: (
        Keyword,
        Option<(Symbol, BlockIdentifier)>,
        Vec<BlockItemDeclaration>,
        Vec<StatementOrNull>,
        Keyword,
        Option<(Symbol, BlockIdentifier)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct ParBlock {
    pub nodes: (
        Keyword,
        Option<(Symbol, BlockIdentifier)>,
        Vec<BlockItemDeclaration>,
        Vec<StatementOrNull>,
        JoinKeyword,
        Option<(Symbol, BlockIdentifier)>,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub enum JoinKeyword {
    Join(Box<Keyword>),
    JoinAny(Box<Keyword>),
    JoinNone(Box<Keyword>),
}
