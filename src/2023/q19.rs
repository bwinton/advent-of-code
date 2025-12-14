//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of, u32},
    multi::{many1, separated_list1},
};

static INPUT: &str = include_str!("data/q19.data");

#[derive(Debug)]
enum Rule<'a> {
    Condition {
        variable: &'a str,
        op: char,
        value: u32,
        destination: &'a str,
    },
    Default {
        destination: &'a str,
    },
}
impl Rule<'_> {
    fn matches(&self, rating: &Rating) -> bool {
        match self {
            Rule::Default { destination: _ } => true,
            Rule::Condition {
                variable,
                op,
                value,
                destination: _,
            } => {
                let test = rating.values.get(variable).unwrap();
                match op {
                    '<' => test < value,
                    '>' => test > value,
                    _ => {
                        panic!("Unknown op! {}", op);
                    }
                }
            }
        }
    }

    fn get_destination(&self) -> &str {
        match self {
            Rule::Condition {
                variable: _,
                op: _,
                value: _,
                destination,
            } => destination,
            Rule::Default { destination } => destination,
        }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

#[derive(Debug)]
struct Rating<'a> {
    values: HashMap<&'a str, u32>,
}
impl Rating<'_> {
    fn accepted(&self, workflows: &HashMap<&str, Workflow>) -> bool {
        let mut curr = "in";
        while curr != "R" && curr != "A" {
            let workflow = workflows.get(curr).unwrap();
            for rule in &workflow.rules {
                if rule.matches(self) {
                    curr = rule.get_destination();
                    break;
                }
            }
        }
        curr == "A"
    }

    fn value(&self) -> u32 {
        self.values.values().sum()
    }
}

fn condition(i: &str) -> IResult<&str, Rule<'_>> {
    // a<2006:qkq OR rfg
    let (input, (variable, op, value, _, destination)) =
        (alpha1, one_of("<>"), u32, tag(":"), alpha1).parse(i)?;
    Ok((
        input,
        Rule::Condition {
            variable,
            op,
            value,
            destination,
        },
    ))
}

fn default(i: &str) -> IResult<&str, Rule<'_>> {
    // rfg
    let (input, destination) = alpha1(i)?;
    Ok((input, Rule::Default { destination }))
}

fn workflow(i: &str) -> IResult<&str, Workflow<'_>> {
    // px{a<2006:qkq,m>2090:A,rfg}
    let (input, (name, _, rules, _, _)) = (
        alpha1,
        tag("{"),
        separated_list1(tag(","), alt((condition, default))),
        tag("}"),
        newline,
    )
        .parse(i)?;
    Ok((input, Workflow { name, rules }))
}

fn rating(i: &str) -> IResult<&str, Rating<'_>> {
    // {x=1102,m=1249,a=1825,s=2027}
    let (input, (_, x, _, m, _, a, _, s, _, _)) = (
        tag("{x="),
        u32,
        tag(",m="),
        u32,
        tag(",a="),
        u32,
        tag(",s="),
        u32,
        tag("}"),
        newline,
    )
        .parse(i)?;
    let values = hashmap! {
        "x" => x,
        "m" => m,
        "a" => a,
        "s" => s,
    };
    Ok((input, Rating { values }))
}

fn parser(i: &str) -> IResult<&str, (HashMap<&str, Workflow<'_>>, Vec<Rating<'_>>)> {
    let (input, (workflows, _, ratings)) = (many1(workflow), newline, many1(rating)).parse(i)?;
    let workflows = HashMap::from_iter(workflows.into_iter().map(|item| (item.name, item)));
    Ok((input, (workflows, ratings)))
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let (workflows, ratings) = parser(data).unwrap().1;
    for rating in ratings {
        if rating.accepted(&workflows) {
            rv += rating.value();
        }
    }
    rv as usize
}

fn process_data_b(data: &str) -> usize {
    let (workflows, _) = parser(data).unwrap().1;
    let mut states = vec![(
        "in",
        hashmap! {
            "x" => (1u64, 4000u64),
            "m" => (1, 4000),
            "a" => (1, 4000),
            "s" => (1, 4000),
        },
    )];
    let mut accepts = vec![];
    while let Some((curr, mut state)) = states.pop() {
        // let (curr, mut state) = states.pop().unwrap();
        if curr == "A" {
            // Add this to the list of accepts, and skip them.
            accepts.push(state);
            continue;
        } else if curr == "R" {
            // skip these, cause they're rejected.
            continue;
        }
        let workflow = workflows.get(curr).unwrap();
        let mut next = vec![];
        for rule in &workflow.rules {
            match rule {
                Rule::Default { destination } => {
                    // In the default case, everything moves to the destination.
                    next.push((*destination, state));
                    break;
                }
                Rule::Condition {
                    variable,
                    op,
                    value,
                    destination,
                } => {
                    let destination = *destination;
                    let value = *value as u64;
                    // split these into success and rest.
                    let mut next_state = state.clone();
                    let range = state.get_mut(variable).unwrap();
                    match op {
                        '<' => {
                            // test < value,
                            if range.1 < value {
                                // we're all in, so don't need to check another rule.
                                next.push((destination, state));
                                break;
                            } else if range.0 >= value {
                                // we're all out, so need to check the next rule.
                                continue;
                            }
                            // The value falls in the range, so split it.
                            next_state.get_mut(variable).unwrap().1 = value - 1;
                            next.push((destination, next_state));
                            range.0 = value;
                        }
                        '>' => {
                            // test > value,
                            if range.0 > value {
                                // we're all in, so don't need to check another rule.
                                next.push((destination, state));
                                break;
                            } else if range.1 <= value {
                                // we're all out, so need to check the next rule.
                                continue;
                            }
                            // The value falls in the range, so split it.
                            next_state.get_mut(variable).unwrap().0 = value + 1;
                            next.push((destination, next_state));
                            range.1 = value;
                        }
                        _ => {
                            panic!("Unknown op! {}", op);
                        }
                    };
                }
            }
        }
        states.append(&mut next);
    }
    let mut rv = 0;
    for accept in accepts {
        rv += accept
            .values()
            .map(|(low, high)| high - low + 1)
            .product::<u64>();
    }
    rv as usize
}

//-----------------------------------------------------
// Questions.

q_impl!("19");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}
    
    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}
    "
        )),
        19114
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "
    px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}
    
    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}
    "
        )),
        167409079868000
    );
}
