//-----------------------------------------------------
// Setup.

use regex::Regex;
use std::collections::HashMap;

static INPUT: &str = include_str!("data/q19.data");
static USE_RE: bool = true;

#[derive(Debug, Clone)]
enum Value {
    Char(char),
    Order(Vec<String>),
    Choice(Vec<String>, Vec<String>),
}

impl Value {
    fn to_regex(&self, rules: &HashMap<&str, Value>) -> String {
        match self {
            Value::Char(c) => c.to_string(),
            Value::Order(items) => {
                let mut rv = String::new();
                for rule in items {
                    rv += &rules[rule.as_str()].to_regex(rules);
                }
                rv
            }
            Value::Choice(first, second) => {
                let mut rv = String::new();
                rv.push('(');
                for rule in first {
                    rv += &rules[rule.as_str()].to_regex(rules);
                }
                rv.push('|');
                for rule in second {
                    rv += &rules[rule.as_str()].to_regex(rules);
                }
                rv.push(')');
                rv
            }
        }
    }
}

fn multi_match(
    prefix: &str,
    message: &str,
    index: usize,
    rules: &HashMap<&str, Value>,
    values: Vec<String>,
) -> Vec<usize> {
    let mut rv = vec![index];
    for rule in values {
        let curr = rv.clone();
        rv = vec![];
        for index in curr {
            let results = matches(prefix, message, index, &rule, rules);
            rv.extend(results);
        }
    }
    rv
}

fn matches(
    prefix: &str,
    message: &str,
    index: usize,
    rule: &str,
    rules: &HashMap<&str, Value>,
) -> Vec<usize> {
    let mut rv = vec![];
    match rules[rule].clone() {
        Value::Char(c) => {
            if message.chars().nth(index) == Some(c) {
                rv.push(index + 1);
            }
        }

        Value::Order(values) => {
            let mut new_prefix = prefix.to_string();
            new_prefix.push_str("  ");
            rv.extend(multi_match(&new_prefix, message, index, rules, values));
        }
        Value::Choice(first, second) => {
            let mut new_prefix = prefix.to_string();
            new_prefix.push_str("  ");
            let first_rv = multi_match(&new_prefix, message, index, rules, first);
            rv.extend(&first_rv);
            let second_rv = multi_match(&new_prefix, message, index, rules, second);
            rv.extend(&second_rv);
        }
    }
    rv
}

fn is_valid(message: &str, rules: &HashMap<&str, Value>) -> bool {
    let rv = matches("", message, 0, "0", rules);
    for len in rv {
        if len == message.len() {
            return true;
        }
    }
    false
}

fn process_data_a(data: &str) -> usize {
    let mut rules = HashMap::new();
    let mut messages = vec![];
    let mut processing_rules = true;
    for line in data.lines() {
        if line.is_empty() {
            processing_rules = false;
            continue;
        }
        if processing_rules {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap();
            let rest = parts.next().unwrap();

            let parts: Vec<_> = rest.split(' ').map(|x| x.to_string()).collect();
            let value = if parts.len() == 1 && (parts[0] == "\"a\"" || parts[0] == "\"b\"") {
                Value::Char(parts[0].chars().nth(1).unwrap())
            } else if parts.contains(&"|".to_string()) {
                let (index, _) = parts.iter().enumerate().find(|&(_, x)| x == "|").unwrap();
                let (first, second) = parts.split_at(index);
                Value::Choice(Vec::from(first), Vec::from(&second[1..]))
            } else {
                Value::Order(parts.clone())
            };

            rules.insert(key, value);
        } else {
            messages.push(line.to_string());
        }
    }

    messages
        .into_iter()
        .filter(|message| is_valid(message, &rules))
        .count()
}

fn process_data_b(data: &str) -> usize {
    let mut rules = HashMap::new();
    let mut messages = vec![];
    let mut processing_rules = true;
    for line in data.lines() {
        if line.is_empty() {
            processing_rules = false;
            continue;
        }
        if processing_rules {
            let mut parts = line.split(": ");
            let key = parts.next().unwrap();
            let rest = parts.next().unwrap();

            let parts: Vec<_> = rest.split(' ').map(|x| x.to_string()).collect();
            let value = if parts.len() == 1 && (parts[0] == "\"a\"" || parts[0] == "\"b\"") {
                Value::Char(parts[0].chars().nth(1).unwrap())
            } else if parts.contains(&"|".to_string()) {
                let (index, _) = parts.iter().enumerate().find(|&(_, x)| x == "|").unwrap();
                let (first, second) = parts.split_at(index);
                Value::Choice(Vec::from(first), Vec::from(&second[1..]))
            } else {
                Value::Order(parts.clone())
            };

            rules.insert(key, value);
        } else {
            messages.push(line.to_string());
        }
    }

    rules.insert(
        "8",
        Value::Choice(vec!["42".to_string()], vec![
            "42".to_string(),
            "801".to_string(),
        ]),
    );
    rules.insert(
        "801",
        Value::Choice(vec!["42".to_string()], vec![
            "42".to_string(),
            "802".to_string(),
        ]),
    );
    rules.insert(
        "802",
        Value::Choice(vec!["42".to_string()], vec![
            "42".to_string(),
            "803".to_string(),
        ]),
    );
    rules.insert(
        "803",
        Value::Choice(vec!["42".to_string()], vec![
            "42".to_string(),
            "804".to_string(),
        ]),
    );
    rules.insert("804", Value::Order(vec!["42".to_string()]));

    rules.insert(
        "11",
        Value::Choice(vec!["42".to_string(), "31".to_string()], vec![
            "42".to_string(),
            "1101".to_string(),
            "31".to_string(),
        ]),
    );
    rules.insert(
        "1101",
        Value::Choice(vec!["42".to_string(), "31".to_string()], vec![
            "42".to_string(),
            "1102".to_string(),
            "31".to_string(),
        ]),
    );
    rules.insert(
        "1102",
        Value::Choice(vec!["42".to_string(), "31".to_string()], vec![
            "42".to_string(),
            "1103".to_string(),
            "31".to_string(),
        ]),
    );
    rules.insert(
        "1103",
        Value::Choice(vec!["42".to_string(), "31".to_string()], vec![
            "42".to_string(),
            "1104".to_string(),
            "31".to_string(),
        ]),
    );
    rules.insert(
        "1104",
        Value::Order(vec!["42".to_string(), "31".to_string()]),
    );

    let mut regex = String::new();
    regex.push('^');
    regex += &rules[&"0"].to_regex(&rules);
    regex.push('$');
    let matcher: Regex = Regex::new(&regex).unwrap();

    if USE_RE {
        messages
            .into_iter()
            .filter(|message| matcher.is_match(message))
            .count()
    } else {
        messages
            .into_iter()
            .filter(|message| is_valid(message, &rules))
            .count()
    }
}

//-----------------------------------------------------
// Questions.

q_impl!("19");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(
            "0: 1 2
1: 4 5 | 4
2: 5
4: \"a\"
5: \"b\"

ab"
        ),
        1
    );

    assert_eq!(
        process_data_a(
            "0: \"a\"

a
aa
b"
        ),
        1
    );

    assert_eq!(
        process_data_a(
            "0: 4 5
4: \"a\"
5: \"b\"

ab
b
aa"
        ),
        1
    );

    assert_eq!(
        process_data_a(
            "0: 4 | 5
4: \"a\"
5: \"b\"

a
b
aa"
        ),
        2
    );

    assert_eq!(
        process_data_a(
            "0: 1 | 5
1: 4
4: \"a\"
5: \"b\"

a
b
aa"
        ),
        2
    );

    assert_eq!(
        process_data_a(
            "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb"
        ),
        1
    );

    assert_eq!(
        process_data_a(
            "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb"
        ),
        2
    );

    assert_eq!(
        process_data_a(
            "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"
        ),
        3
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(
            "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

bbabbbbaabaabba
ababaaaaaabaaab
ababaaaaabbbaba
aaaabbaaaabbaaa
aaaaabbaabaaaaababaa
bbbbbbbaaaabbbbaaabbabaaa
baabbaaaabbaaaababbaababb
babbbbaabbbbbabbbbbbaabaaabaaa
abbbbabbbbaaaababbbbbbaaaababb
babaaabbbaaabaababbaabababaaab
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
bbbababbbbaaaaaaaabbababaaababaabab
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
        ),
        12
    );
}
