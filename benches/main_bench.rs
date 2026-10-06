use divan::prelude::*;
use identity_hash::IntSet;
use rand::distr::Uniform;
use rand::prelude::*;

fn main() {
    divan::main();
}

const SEED: u64 = 42;
const ELEMENTS_NUMBER: u64 = 100;

fn make_generator(unique_percent: u64) -> impl FnMut() -> Vec<u64> {
    let unique_count = (ELEMENTS_NUMBER * unique_percent / 100).max(1);
    let range = 0..unique_count;
    let mut rng = SmallRng::seed_from_u64(SEED);
    move || {
        (&mut rng)
            .sample_iter(Uniform::try_from(range.clone()).unwrap())
            .take(ELEMENTS_NUMBER as usize)
            .collect()
    }
}

#[divan::bench(args = [100, 90, 80, 70, 60, 50, 40, 30, 20, 10])]
fn vec_push_dedup_bench(bencher: Bencher, unique_percent: u64) {
    bencher
        .with_inputs(make_generator(unique_percent))
        .bench_local_refs(|values| {
            let mut v: Vec<u64> = Vec::default();
            for i in values {
                v.push(*i);
            }
            v.sort_unstable();
            v.dedup();
            black_box(v)
        })
}

#[divan::bench(args = [100, 90, 80, 70, 60, 50, 40, 30, 20, 10])]
fn intset_insert_bench(bencher: Bencher, unique_percent: u64) {
    bencher
        .with_inputs(make_generator(unique_percent))
        .bench_local_refs(|values| {
            let mut set: IntSet<u64> = IntSet::default();
            for i in values {
                set.insert(*i);
            }
            black_box(set)
        })
}
