use criterion::{black_box, criterion_group, criterion_main, Criterion};
use caesar_cipher_cracker::caesar_shift;

fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("caesar_shift 5", |b| {
        b.iter(|| {
            caesar_shift(
                black_box("P ht uva h ipn mhu vm Jhlzhy Jpwoly"),
                black_box(5),
            )
        })
    });

    c.bench_function("caesar_shift 10", |b| {
        b.iter(|| {
            caesar_shift(
                black_box("P ht uva h ipn mhu vm Jhlzhy Jpwoly"),
                black_box(10),
            )
        })
    });

    c.bench_function("caesar_shift 15", |b| {
        b.iter(|| {
            caesar_shift(
                black_box("P ht uva h ipn mhu vm Jhlzhy Jpwoly"),
                black_box(15),
            )
        })
    });

    c.bench_function("caesar_shift 20", |b| {
        b.iter(|| {
            caesar_shift(
                black_box("P ht uva h ipn mhu vm Jhlzhy Jpwoly"),
                black_box(20),
            )
        })
    });

    c.bench_function("caesar_shift 25", |b| {
        b.iter(|| {
            caesar_shift(
                black_box("P ht uva h ipn mhu vm Jhlzhy Jpwoly"),
                black_box(25),
            )
        })
    });

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
