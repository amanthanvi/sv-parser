use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node)]
pub enum Number {
    IntegralNumber(Box<IntegralNumber>),
    RealNumber(Box<RealNumber>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub enum IntegralNumber {
    DecimalNumber(Box<DecimalNumber>),
    OctalNumber(Box<OctalNumber>),
    BinaryNumber(Box<BinaryNumber>),
    HexNumber(Box<HexNumber>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub enum DecimalNumber {
    UnsignedNumber(Box<UnsignedNumber>),
    BaseUnsigned(Box<DecimalNumberBaseUnsigned>),
    BaseXNumber(Box<DecimalNumberBaseXNumber>),
    BaseZNumber(Box<DecimalNumberBaseZNumber>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct DecimalNumberBaseUnsigned {
    pub nodes: (Option<Size>, DecimalBase, UnsignedNumber),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct DecimalNumberBaseXNumber {
    pub nodes: (Option<Size>, DecimalBase, XNumber),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct DecimalNumberBaseZNumber {
    pub nodes: (Option<Size>, DecimalBase, ZNumber),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct BinaryNumber {
    pub nodes: (Option<Size>, BinaryBase, BinaryValue),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct OctalNumber {
    pub nodes: (Option<Size>, OctalBase, OctalValue),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct HexNumber {
    pub nodes: (Option<Size>, HexBase, HexValue),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub enum Sign {
    Plus(Box<Symbol>),
    Minus(Box<Symbol>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct Size {
    pub nodes: (NonZeroUnsignedNumber,),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct NonZeroUnsignedNumber {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub enum RealNumber {
    FixedPointNumber(Box<FixedPointNumber>),
    Floating(Box<RealNumberFloating>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct RealNumberFloating {
    pub nodes: (
        UnsignedNumber,
        Option<(Symbol, UnsignedNumber)>,
        Exp,
        Option<Sign>,
        UnsignedNumber,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct FixedPointNumber {
    pub nodes: (UnsignedNumber, Symbol, UnsignedNumber),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct Exp {
    pub nodes: (Symbol,),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct UnsignedNumber {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct BinaryValue {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct OctalValue {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct HexValue {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct DecimalBase {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct BinaryBase {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct OctalBase {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct HexBase {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct XNumber {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct ZNumber {
    pub nodes: (Locate, Vec<WhiteSpace>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct UnbasedUnsizedLiteral {
    pub nodes: (Symbol,),
}
