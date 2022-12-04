use crate::*;

// -----------------------------------------------------------------------------

#[tracable_parser]
#[packrat_parser]
pub(crate) fn continuous_assign(s: Span) -> IResult<Span, ContinuousAssign> {
    alt((continuous_assign_net, continuous_assign_variable))(s)
}

#[tracable_parser]
#[packrat_parser]
pub(crate) fn continuous_assign_net(s: Span) -> IResult<Span, ContinuousAssign> {
    let (s, a) = keyword("assign")(s)?;
    let (s, b) = opt(drive_strength)(s)?;
    let (s, c) = opt(delay3)(s)?;
    let (s, d) = list_of_net_assignments(s)?;
    let (s, e) = symbol(";")(s)?;

    Ok((
        s,
        ContinuousAssign::Net(Box::new(ContinuousAssignNet {
            nodes: (a, b, c, d, e),
        })),
    ))
}

#[tracable_parser]
#[packrat_parser]
pub(crate) fn continuous_assign_variable(s: Span) -> IResult<Span, ContinuousAssign> {
    let (s, a) = keyword("assign")(s)?;
    let (s, b) = opt(delay_control)(s)?;
    let (s, c) = list_of_variable_assignments(s)?;
    let (s, d) = symbol(";")(s)?;

    Ok((
        s,
        ContinuousAssign::Variable(Box::new(ContinuousAssignVariable {
            nodes: (a, b, c, d),
        })),
    ))
}

#[recursive_parser]
#[tracable_parser]
#[packrat_parser]
pub(crate) fn list_of_net_assignments(s: Span) -> IResult<Span, ListOfNetAssignments> {
    let (s, a) = list(symbol(","), net_assignment)(s)?;
    Ok((s, ListOfNetAssignments { nodes: (a,) }))
}

#[recursive_parser]
#[tracable_parser]
#[packrat_parser]
pub(crate) fn list_of_variable_assignments(s: Span) -> IResult<Span, ListOfVariableAssignments> {
    let (s, a) = list(symbol(","), variable_assignment)(s)?;
    Ok((s, ListOfVariableAssignments { nodes: (a,) }))
}

#[tracable_parser]
#[packrat_parser]
pub(crate) fn net_alias(s: Span) -> IResult<Span, NetAlias> {
    let (s, a) = keyword("alias")(s)?;
    let (s, b) = net_lvalue(s)?;
    let (s, c) = symbol("=")(s)?;
    let (s, d) = list(symbol("="), net_lvalue)(s)?;
    let (s, e) = symbol(";")(s)?;

    Ok((
        s,
        NetAlias {
            nodes: (a, b, c, d, e),
        },
    ))
}

#[recursive_parser]
#[tracable_parser]
#[packrat_parser]
pub(crate) fn net_assignment(s: Span) -> IResult<Span, NetAssignment> {
    let (s, a) = net_lvalue(s)?;
    let (s, b) = symbol("=")(s)?;
    let (s, c) = expression(s)?;

    Ok((s, NetAssignment { nodes: (a, b, c) }))
}
