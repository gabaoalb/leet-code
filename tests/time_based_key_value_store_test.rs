use leet_code::time_based_key_value_store::TimeMap;

#[test]
fn case_1() {
    let mut obj = TimeMap::new();
    obj.set("foo".to_string(), "bar".to_string(), 1);
    let ret_1: String = obj.get("foo".to_string(), 1);
    assert_eq!(ret_1, "bar");
    let ret_2: String = obj.get("foo".to_string(), 3);
    assert_eq!(ret_2, "bar");
    obj.set("foo".to_string(), "bar2".to_string(), 4);
    let ret_3: String = obj.get("foo".to_string(), 4);
    assert_eq!(ret_3, "bar2");
    let ret_4: String = obj.get("foo".to_string(), 5);
    assert_eq!(ret_4, "bar2")
}

#[test]
fn case_2() {
    let mut obj = TimeMap::new();
    obj.set("love".to_string(), "high".to_string(), 10);
    obj.set("love".to_string(), "low".to_string(), 20);

    let ret_1: String = obj.get("love".to_string(), 5);
    assert_eq!(ret_1, "");

    let ret_2: String = obj.get("love".to_string(), 10);
    assert_eq!(ret_2, "high");

    let ret_3: String = obj.get("love".to_string(), 15);
    assert_eq!(ret_3, "high");

    let ret_4: String = obj.get("love".to_string(), 20);
    assert_eq!(ret_4, "low");

    let ret_5: String = obj.get("love".to_string(), 25);
    assert_eq!(ret_5, "low")
}
