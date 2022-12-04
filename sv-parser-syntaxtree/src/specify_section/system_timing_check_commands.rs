use crate::*;

// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Node)]
pub enum SystemTimingCheck {
    SetupTimingCheck(Box<SetupTimingCheck>),
    HoldTimingCheck(Box<HoldTimingCheck>),
    SetupholdTimingCheck(Box<SetupholdTimingCheck>),
    RecoveryTimingCheck(Box<RecoveryTimingCheck>),
    RemovalTimingCheck(Box<RemovalTimingCheck>),
    RecremTimingCheck(Box<RecremTimingCheck>),
    SkewTimingCheck(Box<SkewTimingCheck>),
    TimeskewTimingCheck(Box<TimeskewTimingCheck>),
    FullskewTimingCheck(Box<FullskewTimingCheck>),
    PeriodTimingCheck(Box<PeriodTimingCheck>),
    WidthTimingCheck(Box<WidthTimingCheck>),
    NochangeTimingCheck(Box<NochangeTimingCheck>),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct SetupTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            DataEvent,
            Symbol,
            ReferenceEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct HoldTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct SetupholdTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Symbol,
            TimingCheckLimit,
            Option<(
                Symbol,
                Option<Notifier>,
                Option<(
                    Symbol,
                    Option<TimestampCondition>,
                    Option<(
                        Symbol,
                        Option<TimecheckCondition>,
                        Option<(
                            Symbol,
                            Option<DelayedReference>,
                            Option<(Symbol, Option<DelayedData>)>,
                        )>,
                    )>,
                )>,
            )>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct RecoveryTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct RemovalTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct RecremTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Symbol,
            TimingCheckLimit,
            Option<(
                Symbol,
                Option<Notifier>,
                Option<(
                    Symbol,
                    Option<TimestampCondition>,
                    Option<(
                        Symbol,
                        Option<TimecheckCondition>,
                        Option<(
                            Symbol,
                            Option<DelayedReference>,
                            Option<(Symbol, Option<DelayedData>)>,
                        )>,
                    )>,
                )>,
            )>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct SkewTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct TimeskewTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Option<(
                Symbol,
                Option<Notifier>,
                Option<(
                    Symbol,
                    Option<EventBasedFlag>,
                    Option<(Symbol, Option<RemainActiveFlag>)>,
                )>,
            )>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct FullskewTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            TimingCheckLimit,
            Symbol,
            TimingCheckLimit,
            Option<(
                Symbol,
                Option<Notifier>,
                Option<(
                    Symbol,
                    Option<EventBasedFlag>,
                    Option<(Symbol, Option<RemainActiveFlag>)>,
                )>,
            )>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct PeriodTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ControlledReferenceEvent,
            Symbol,
            TimingCheckLimit,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct WidthTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ControlledReferenceEvent,
            Symbol,
            TimingCheckLimit,
            Symbol,
            Threshold,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}

#[derive(Clone, Debug, PartialEq, Node)]
pub struct NochangeTimingCheck {
    pub nodes: (
        Keyword,
        Paren<(
            ReferenceEvent,
            Symbol,
            DataEvent,
            Symbol,
            StartEdgeOffset,
            Symbol,
            EndEdgeOffset,
            Option<(Symbol, Option<Notifier>)>,
        )>,
        Symbol,
    ),
}
