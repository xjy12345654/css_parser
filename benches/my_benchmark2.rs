//使用criterion库 测试对比不同函数的性能
//设置cargo.toml
// [[bench]]
//name = "my_benchmark2"
//harness = false
//最后运行 cargo bench
use criterion::{Criterion, criterion_group, criterion_main};
use rayon::prelude::*;
use std::fs;
use std::hint::black_box;
use std::path::{Path, PathBuf};

fn process_single_file(file_path: &Path) -> bool {
    let file_name = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    // println!("unfilter_{}",file_name);
    if file_name.ends_with("_conv_rem") || file_name.ends_with("_conv_vw") {
        return false; // 跳过处理
    }

    // 模拟文件处理逻辑
    true
}

// 延迟过滤文件名
fn find_css_files_unfiltered(dir: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut css_files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            css_files.extend(find_css_files_unfiltered(path.to_str().unwrap())?);
        } else if let Some(ext) = path.extension() {
            if ext.to_str() == Some("css") {
                css_files.push(path);
            }
        }
    }
    Ok(css_files)
}

// 延迟过滤文件名的并行操作
fn process_files_unfiltered(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let css_files = find_css_files_unfiltered(dir)?;
    css_files.par_iter().for_each(|file_path| {
        process_single_file(file_path);
    });
    Ok(())
}

fn process_single_file2(file_path: &Path) -> bool {
    let file_name = file_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default();
    // println!("filter_{}",file_name);
    // 模拟文件处理逻辑
    true
}

// 提前过滤文件名
fn find_css_files_filtered(dir: &str) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut css_files = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            css_files.extend(find_css_files_filtered(path.to_str().unwrap())?);
        } else if let Some(ext) = path.extension() {
            if ext.to_str() == Some("css") {
                let file_name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                if !file_name.ends_with("_conv_rem") && !file_name.ends_with("_conv_vw") {
                    css_files.push(path);
                }
            }
        }
    }
    Ok(css_files)
}

fn process_files_filtered(dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let css_files = find_css_files_filtered(dir)?;
    css_files.par_iter().for_each(|file_path| {
        process_single_file2(file_path);
    });
    Ok(())
}

fn benchmark(c: &mut Criterion) {
    // let path = Path::new("example_conv_rem.css");
    let test_dir = "C:/Users/xjy12/Desktop/test";


    c.bench_function("process_files_unfiltered", |b| {
        b.iter(|| {
            let result = process_files_unfiltered(test_dir);
            let _ = black_box(result);
        })
    });

  
    c.bench_function("process_files_filtered", |b| {
        b.iter(|| {
            let result = process_files_filtered(test_dir);
            let _ = black_box(result);
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
