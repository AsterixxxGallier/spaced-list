// #![feature(portable_simd)]
//
// use std::ops::{Add, AddAssign};
// use std::simd::{Simd, SimdElement, SupportedLaneCount};
// use std::simd::LaneCount;
// use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
//
// #[inline]
// fn add_as_simd<const SIZE: usize, N>(a: N, b: N) -> Simd<N, SIZE>
// 	where LaneCount<SIZE>: SupportedLaneCount,
// 	N: SimdElement + Copy + Add<Output = N>,
// 	Simd<N, SIZE>: Add<Output = Simd<N, SIZE>> {
// 	let a_simd: Simd<N, SIZE> = [a; SIZE].into();
// 	let b_simd: Simd<N, SIZE> = Simd::splat(b);
// 	// <Simd<N, SIZE> as Add>::add(a, b);
// 	Simd::splat(1).add(Simd::splat(2));
// 	let sum = a_simd + b_simd;
// 	sum
// }
//
// fn add_as_array<const SIZE: usize, N>(a: N, b: N) -> [N; SIZE]
// 	where LaneCount<SIZE>: SupportedLaneCount,
// 	N: SimdElement + Copy + AddAssign {
// 	let mut array = [a; SIZE];
// 	for x in &mut array {
// 		*x += b
// 	}
// 	array
// }
//
// fn criterion_benchmark(c: &mut Criterion) {
// 	// c.bench_function("add as simd 30 20", |b| b.iter(||
// 	// 	add_as_simd(black_box(30), black_box(20))));
// 	// c.bench_function("add as array 30 20", |b| b.iter(||
// 	// 	add_as_array(black_box(30), black_box(20))));
//
// 	let mut group = c.benchmark_group("add 20 to 30 smol optimized 2");
//
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u16", 1), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<1, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u16", 2), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<2, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u16", 4), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<4, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u16", 8), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<8, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u16", 16), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<16, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u16", 32), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<32, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u16", 64), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<64, u16>(black_box(30), black_box(20)));
// 	// });
//
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u16", 1), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<1, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u16", 2), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<2, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u16", 4), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<4, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u16", 8), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<8, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u16", 16), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<16, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u16", 32), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<32, u16>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u16", 64), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<64, u16>(black_box(30), black_box(20)));
// 	// });
//
//
// 	group.bench_with_input(BenchmarkId::new("add_as_simd u32", 1), &0, |b, &_| {
// 		b.iter(|| add_as_simd::<1, u32>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_simd u32", 2), &0, |b, &_| {
// 		b.iter(|| add_as_simd::<2, u32>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_simd u32", 4), &0, |b, &_| {
// 		b.iter(|| add_as_simd::<4, u32>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_simd u32", 8), &0, |b, &_| {
// 		b.iter(|| add_as_simd::<8, u32>(black_box(30), black_box(20)));
// 	});
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u32", 16), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<16, u32>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u32", 32), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<32, u32>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u32", 64), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<64, u32>(black_box(30), black_box(20)));
// 	// });
//
// 	group.bench_with_input(BenchmarkId::new("add_as_array u32", 1), &0, |b, &_| {
// 		b.iter(|| add_as_array::<1, u32>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_array u32", 2), &0, |b, &_| {
// 		b.iter(|| add_as_array::<2, u32>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_array u32", 4), &0, |b, &_| {
// 		b.iter(|| add_as_array::<4, u32>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_array u32", 8), &0, |b, &_| {
// 		b.iter(|| add_as_array::<8, u32>(black_box(30), black_box(20)));
// 	});
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u32", 16), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<16, u32>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u32", 32), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<32, u32>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u32", 64), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<64, u32>(black_box(30), black_box(20)));
// 	// });
//
//
// 	group.bench_with_input(BenchmarkId::new("add_as_simd u64", 1), &0, |b, &_| {
// 		b.iter(|| add_as_simd::<1, u64>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_simd u64", 2), &0, |b, &_| {
// 		b.iter(|| add_as_simd::<2, u64>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_simd u64", 4), &0, |b, &_| {
// 		b.iter(|| add_as_simd::<4, u64>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_simd u64", 8), &0, |b, &_| {
// 		b.iter(|| add_as_simd::<8, u64>(black_box(30), black_box(20)));
// 	});
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u64", 16), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<16, u64>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u64", 32), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<32, u64>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_simd u64", 64), &0, |b, &_| {
// 	// 	b.iter(|| add_as_simd::<64, u64>(black_box(30), black_box(20)));
// 	// });
//
// 	group.bench_with_input(BenchmarkId::new("add_as_array u64", 1), &0, |b, &_| {
// 		b.iter(|| add_as_array::<1, u64>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_array u64", 2), &0, |b, &_| {
// 		b.iter(|| add_as_array::<2, u64>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_array u64", 4), &0, |b, &_| {
// 		b.iter(|| add_as_array::<4, u64>(black_box(30), black_box(20)));
// 	});
// 	group.bench_with_input(BenchmarkId::new("add_as_array u64", 8), &0, |b, &_| {
// 		b.iter(|| add_as_array::<8, u64>(black_box(30), black_box(20)));
// 	});
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u64", 16), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<16, u64>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u64", 32), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<32, u64>(black_box(30), black_box(20)));
// 	// });
// 	// group.bench_with_input(BenchmarkId::new("add_as_array u64", 64), &0, |b, &_| {
// 	// 	b.iter(|| add_as_array::<64, u64>(black_box(30), black_box(20)));
// 	// });
//
// 	group.finish();
// }
//
// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
