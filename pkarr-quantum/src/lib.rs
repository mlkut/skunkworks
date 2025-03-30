#![feature(test)]

extern crate test;

fn random_bytes(num_bytes: usize) -> Vec<u8> {
    let mut buf = vec![0u8; num_bytes];
    getrandom::getrandom(&mut buf).expect("getrandom");

    buf
}

fn hash_many(seed: &[u8], count: usize) -> blake3::Hash {
    let mut result = blake3::hash(&seed);

    for _ in 1..count {
        result = blake3::hash(result.as_bytes());
    }

    result
}

#[cfg(test)]
mod tests {
    use tests::test::Bencher;

    use super::*;

    #[bench]
    fn bench_hash_1(b: &mut Bencher) {
        b.iter(|| {
            let seed = random_bytes(32);
            hash_many(&seed, 1);
        });
    }

    #[bench]
    fn bench_hash_100(b: &mut Bencher) {
        b.iter(|| {
            let seed = random_bytes(32);

            hash_many(&seed, 100);
        });
    }

    #[bench]
    fn bench_hash_1000(b: &mut Bencher) {
        b.iter(|| {
            let seed = random_bytes(32);

            hash_many(&seed, 1000);
        });
    }

    #[bench]
    fn bench_hash_1000_000(b: &mut Bencher) {
        b.iter(|| {
            let seed = random_bytes(32);

            hash_many(&seed, 1000_000);
        });
    }

    #[bench]
    fn bench_hash_10_000_000(b: &mut Bencher) {
        b.iter(|| {
            let seed = random_bytes(32);

            hash_many(&seed, 10_000_000);
        });
    }
}
