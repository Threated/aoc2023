fn main() {
    divan::main()
}

#[divan::bench]
fn p2_slow() {
    divan::black_box(include_str!("../input"))
        .lines()
        .flat_map(d1::str_to_num_p2)
        .sum::<u32>();
}

#[divan::bench]
fn p2_fast() {
    divan::black_box(include_str!("../input"))
        .lines()
        .flat_map(d1::str_to_num_p2_fast)
        .sum::<u32>();
}
