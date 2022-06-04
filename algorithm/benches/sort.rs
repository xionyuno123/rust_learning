use criterion::{black_box,criterion_group, criterion_main, Criterion};


use algorithm::compare_sort::*;

lazy_static::lazy_static!(
    static ref INPUT:Vec<Vec<i32>>={
        let mut ret = Vec::new();
        for _i in 0..1000{
            let len= _i*5;
            ret.push(Vec::new());
            for _j in 0..len{
                ret[_i].push(rand::random());
            }
        }

        ret
    };
);



pub fn b1(c: &mut Criterion) {
    c.bench_function("heap_sort", |b| {
        b.iter(|| {
            static mut IDX:usize=0;
            let mut arr=unsafe{
                INPUT.get_unchecked(IDX).clone()
            };
            unsafe{
                IDX+=1;
                IDX%=1000;
            };
            heap_sort(black_box(&mut arr));
        })
    });
}

pub fn b2(c: &mut Criterion) {
    c.bench_function("insert_sort", |b| {
        b.iter(|| {
            static mut IDX:usize=0;
            let mut arr=unsafe{
                INPUT.get_unchecked(IDX).clone()
            };
            unsafe{
                IDX+=1;
                IDX%=1000;
            };
            insert_sort(black_box(&mut arr));
        })
    });
}


pub fn b3(c: &mut Criterion) {
    c.bench_function("merge_sort", |b| {
        b.iter(|| {
            static mut IDX:usize=0;
            let mut arr=unsafe{
                INPUT.get_unchecked(IDX).clone()
            };
            unsafe{
                IDX+=1;
                IDX%=1000;
            };
            merge_sort(black_box(&mut arr));
        })
    });
}

pub fn b4(c: &mut Criterion) {
    c.bench_function("fast_sort", |b| {
        b.iter(|| {
            static mut IDX:usize=0;
            let mut arr=unsafe{
                INPUT.get_unchecked(IDX).clone()
            };
            unsafe{
                IDX+=1;
                IDX%=1000;
            };
            fast_sort(black_box(&mut arr));
        })
    });
}

criterion_group!(benches, b1,b2,b3,b4);
criterion_main!(benches);


