use criterion::{Criterion, criterion_group, criterion_main};
use rand::Rng;

use binary_search::binary_search;

pub fn binary_search_benchmark(c: &mut Criterion) {
    let mut s = vec![0i32; 1_000_000];
    rand::thread_rng().fill(&mut s[..]);
    s.sort();

    let slice: &[_] = &s;
    let mut elements = [0; 1024];
    rand::thread_rng().fill(&mut elements[..]);

    c
        .bench_function(
            "binary_search",
            |b| b.iter(||
                for element in elements {
                    binary_search(slice, &element);
                }
            ),
        )
        .bench_function(
            "iter",
            |b| b.iter(||
                for element in elements {
                    slice.iter().position(|&e| e == element);
                }
            ),
        );
}

criterion_group!(benches, binary_search_benchmark);
criterion_main!(benches);
