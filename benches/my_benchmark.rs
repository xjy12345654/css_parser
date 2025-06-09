use criterion::{ criterion_group, criterion_main, Criterion};
use std::path::Path;
use std::hint::black_box;
fn to_string_method(path: &Path) -> String {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string()
}

fn double_call_method(path: &Path) -> &str {
    path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
}

fn benchmark(c: &mut Criterion) {
    let path = Path::new("example_conv_rem.css");

    // 基准测试 `to_string` 方法
    c.bench_function("to_string_method", |b| {
        b.iter(|| {
            let result = to_string_method(black_box(path));
            black_box(result);
        })
    });

    // 基准测试两次调用 `file_stem` 方法
    c.bench_function("double_call_method", |b| {
        b.iter(|| {
            let result1 = double_call_method(black_box(path));
            let result2 = double_call_method(black_box(path));
            black_box((result1, result2));
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);