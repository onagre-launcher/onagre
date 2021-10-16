use criterion::{criterion_group, criterion_main, Criterion};
use onagre::freedesktop::IconFinder;

fn icon_lookup(c: &mut Criterion) {
    let finder = IconFinder::build("Arc").unwrap();
    c.bench_function("lookup firefox icon", |b| {
        b.iter(|| {
            finder.lookup("firefox", 24);
        })
    });
}

criterion_group!(benches, icon_lookup);
criterion_main!(benches);
