use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node)]
pub enum ProgramItem {
    PortDeclaration(Box<(PortDeclaration, Symbol)>),
    NonPortProgramItem(Box<NonPortProgramItem>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub enum NonPortProgramItem {
    Assign(Box<NonPortProgramItemAssign>),
    Module(Box<NonPortProgramItemModule>),
    Initial(Box<NonPortProgramItemInitial>),
    Final(Box<NonPortProgramItemFinal>),
    Assertion(Box<NonPortProgramItemAssertion>),
    TimeunitsDeclaration(Box<TimeunitsDeclaration>),
    ProgramGenerateItem(Box<ProgramGenerateItem>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct NonPortProgramItemAssign {
    pub nodes: (Vec<AttributeInstance>, ContinuousAssign),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct NonPortProgramItemModule {
    pub nodes: (Vec<AttributeInstance>, ModuleOrGenerateItemDeclaration),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct NonPortProgramItemInitial {
    pub nodes: (Vec<AttributeInstance>, InitialConstruct),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct NonPortProgramItemFinal {
    pub nodes: (Vec<AttributeInstance>, FinalConstruct),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct NonPortProgramItemAssertion {
    pub nodes: (Vec<AttributeInstance>, ConcurrentAssertionItem),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub enum ProgramGenerateItem {
    LoopGenerateConstruct(Box<LoopGenerateConstruct>),
    ConditionalGenerateConstruct(Box<ConditionalGenerateConstruct>),
    GenerateRegion(Box<GenerateRegion>),
    ElaborationSystemTask(Box<ElaborationSystemTask>),
}
