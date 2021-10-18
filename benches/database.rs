use criterion::{criterion_group, criterion_main, Criterion};
use onagre::db::desktop_entry::DesktopEntryEntity;
use onagre::db::Database;

fn open_db(c: &mut Criterion) {
    c.bench_function("open db", |b| {
        b.iter(|| {
            Database::default();
        })
    });
}

fn get_all_desktop_entries(c: &mut Criterion) {
    let db = Database::default();
    c.bench_function("get all history entry", |b| {
        b.iter(|| {
            db.get_all::<DesktopEntryEntity>();
        })
    });
}

criterion_group!(benches, open_db, get_all_desktop_entries);
criterion_main!(benches);
