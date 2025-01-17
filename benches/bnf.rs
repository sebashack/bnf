use bnf::Grammar;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;

fn examples(c: &mut Criterion) {
    c.bench_function("parse postal", |b| {
        let input = std::include_str!("../tests/fixtures/postal_address.terminated.input.bnf");
        b.iter(|| input.parse::<Grammar>().unwrap());
    });

    c.bench_function("generate DNA", |b| {
        let input = "<dna> ::= <base> | <base> <dna>
            <base> ::= \"A\" | \"C\" | \"G\" | \"T\"";
        let grammar: Grammar = input.parse().unwrap();
        b.iter(|| grammar.generate().unwrap());
    });

    let polish_calc_grammar: Grammar = "<product> ::= <number> | <op> <product> <product>
            <op> ::= \"+\" | \"-\" | \"*\" | \"/\"
            <number> ::= \"0\" | \"1\" | \"2\" | \"3\" | \"4\" | \"5\" | \"6\" | \"7\" | \"8\" | \"9\"
        ".parse().unwrap();

    // use pseudo random for consistent metrics
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(0);
    let random_walk_count = 100usize;
    let random_walks: Vec<_> = (0..random_walk_count)
        .map(|_| polish_calc_grammar.generate_seeded(&mut rng).unwrap())
        .collect();

    c.bench_function("parse polish calculator", |b| {
        b.iter(|| {
            let input = random_walks.choose(&mut rng).unwrap();
            let _: Vec<_> = polish_calc_grammar.parse_input(input).collect();
        })
    });
}

criterion_group!(benches, examples);
criterion_main!(benches);
