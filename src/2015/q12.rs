//-----------------------------------------------------
// Setup.

use regex::Regex;
use serde_json::from_str;
use serde_json::Value;

static INPUT: &str = include_str!("data/q12.data");

fn process_data_a(data: &str) -> i64 {
    lazy_static! {
        static ref NUMBERS: Regex = Regex::new(r"-?\d+").unwrap();
    }
    let mut rv = 0;
    for cap in NUMBERS.captures_iter(data) {
        rv += &cap[0].parse().unwrap();
    }
    rv
}

fn get_sum(v: &Value) -> i64 {
    match *v {
        Value::Null | Value::Bool(_) | Value::String(_) => 0,
        Value::Number(ref n) => n.as_i64().unwrap(),
        Value::Array(ref children) => children.iter().map(|x| get_sum(x)).sum(),
        Value::Object(ref children) => {
            if children
                .values()
                .find(|x| *x == &Value::String("red".to_owned()))
                == None
            {
                children.values().map(|x| get_sum(x)).sum()
            } else {
                0
            }
        }
    }
}

fn process_data_b(data: &str) -> i64 {
    let v: Value = from_str(data).unwrap();
    get_sum(&v)
}

//-----------------------------------------------------
// Questions.

q_impl!("12");

#[test]
fn a() {
    assert_eq!(process_data_a("[1,2,3]"), 6);
    assert_eq!(process_data_a("{\"a\":2,\"b\":4}"), 6);
    assert_eq!(process_data_a("[[[3]]]"), 3);
    assert_eq!(process_data_a("{\"a\":{\"b\":4},\"c\":-1}"), 3);
    assert_eq!(process_data_a("{\"a\":[-1,1]}"), 0);
    assert_eq!(process_data_a("[-1,{\"a\":1}]"), 0);
    assert_eq!(process_data_a("[]"), 0);
    assert_eq!(process_data_a("{}"), 0);
}

#[test]
fn b() {
    assert_eq!(process_data_b("[1,2,3]"), 6);
    assert_eq!(process_data_b("[1,{\"c\":\"red\",\"b\":2},3]"), 4);
    assert_eq!(process_data_b("[1,{\"red\":\"c\",\"b\":2},3]"), 6);
    assert_eq!(process_data_b("{\"d\":\"red\",\"e\":[1,2,3,4],\"f\":5}"), 0);
    assert_eq!(process_data_b("[1,\"red\",5]"), 6);
}
