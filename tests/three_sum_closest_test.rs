use leet_code::three_sum_closest::Solution;

// TODO! revisar os casos de teste, principalmente o terceiro, que tem um resultado que é igual ao target, mas não é o mais próximo. Talvez seja necessário adicionar mais casos de teste para cobrir melhor os cenários.

#[test]
#[ignore]
fn case_1() {
    assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2)
}

#[test]
// #[ignore]
fn case_2() {
    assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1), 0)
}

#[test]
#[ignore]
fn case_3() {
    assert_eq!(
        Solution::three_sum_closest(vec![4, 0, 5, -5, 3, 3, 0, -4, -5], -2),
        -2
    )
}
