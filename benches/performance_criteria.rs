use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::collections::HashMap;

fn create_hash_table(n: usize) -> HashMap<usize, usize> {
    let mut addresses = HashMap::with_capacity(n);

    for i in 0..n {
        addresses.insert(i, i);
    }

    addresses
}

fn max_value_in_hash_table(table: &HashMap<usize, usize>) -> Option<usize> {
    let mut max = None;

    for (_key, value) in table {
        match max {
            None => {
                max = Some(*value);
            }
            Some(previous) => {
                if value > &previous {
                    max = Some(*value);
                }
            }
        }
    }

    max
}

fn criterion_benchmark(c: &mut Criterion) {
    // At one point in time I was curious if I could "afford" to do logarithmic work during
    // request_handled. I assumed inserting 1_000_000 elements into a `HashMap` is representative
    // of logarithmic work on 10 million addresses. This benchmark ran in ~40ms or about 25 times
    // per second. Conclusion, `request_handled`, must be sub-logarithmic, probably constant-time.
    c.bench_function("Insert 1 million elements into hash table", |b| {
        b.iter(|| create_hash_table(black_box(1_000_000)))
    });

    // I wanted to know if linear time was "fast enough" for `top_100`. So, I created over a
    // hash table with 100 million elements (for 10x growth) and found the max. This benchmark run
    // in 275ms. This might be "fast enough" depending on the calling code, but it is a long time
    // to be thread-locked in event-loop based web server. Additionally, I would expect trouble if
    // dozens of requests for top_100 came in at the same time. Conclusion, `top_100` should be
    // sub-linear.
    let hash_table = create_hash_table(100_000_000);
    c.bench_with_input(
        BenchmarkId::new(
            "Maximum of a 100 million element hash table",
            "[hash_table size:100_000_000]",
        ),
        &hash_table,
        |b, hash_table| {
            b.iter(|| {
                // Criterion already passed `hash_table` through `black_box`. Convenient...
                max_value_in_hash_table(&hash_table);
            });
        },
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
