use leet_code::group_anagrams::Solution;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
    for group in &mut groups {
        group.sort();
    }
    groups.sort_by(|a, b| a[0].cmp(&b[0]));
    groups
}

#[test]
fn case_1() {
    let input = vec_of_strings!["eat", "tea", "tan", "ate", "nat", "bat"];
    let expected = Solution::group_anagrams(input);
    let actual = vec![vec_of_strings!["bat"], vec_of_strings!["tan", "nat"], vec_of_strings!["eat", "tea", "ate"]];
    assert_eq!(
        normalize(expected),
        normalize(actual)
    )
}

#[test]
fn case_2() {
    let input = vec_of_strings![""];
    let expected = Solution::group_anagrams(input);
    let actual = vec![vec_of_strings![""]];
    assert_eq!(
        normalize(expected),
        normalize(actual)
    )
}

#[test]
fn case_44() {
    let input = vec_of_strings!["a"];
    let expected = Solution::group_anagrams(input);
    let actual = vec![vec_of_strings!["a"]];
    assert_eq!(
        normalize(expected),
        normalize(actual)
    )
}
