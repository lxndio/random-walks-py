use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use randomwalks_lib::dp::builder::DynamicProgramBuilder;
use randomwalks_lib::dp::DynamicPrograms;
use randomwalks_lib::kernel::correlated_rw::CorrelatedRwGenerator;
use randomwalks_lib::kernel::simple_rw::SimpleRwGenerator;
use randomwalks_lib::kernel::Kernel;
use randomwalks_lib::walker::standard::StandardWalker;
use randomwalks_lib::walker::Walker;

pub fn benchmark_dp_simple(c: &mut Criterion) {
    let time_limits = (100..=500).step_by(50);
    let mut group = c.benchmark_group("DP compute simple");

    for time_limit in time_limits {
        group.sample_size(10).bench_with_input(
            BenchmarkId::from_parameter(time_limit),
            &time_limit,
            |b, &time_limit| {
                let mut dp = DynamicProgramBuilder::new()
                    .simple()
                    .time_limit(time_limit)
                    .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
                    .build()
                    .unwrap();

                b.iter(|| dp.compute());
            },
        );
    }
}

pub fn benchmark_dp_correlated(c: &mut Criterion) {
    let time_limits = (100..=500).step_by(50);
    let mut group = c.benchmark_group("DP compute correlated");

    for time_limit in time_limits {
        group.sample_size(10).bench_with_input(
            BenchmarkId::from_parameter(time_limit),
            &time_limit,
            |b, &time_limit| {
                let mut dp = DynamicProgramBuilder::new()
                    .multi()
                    .time_limit(time_limit)
                    .kernels(
                        Kernel::multiple_from_generator(CorrelatedRwGenerator {
                            persistence: 0.05,
                        })
                        .unwrap(),
                    )
                    .build()
                    .unwrap();

                b.iter(|| dp.compute());
            },
        );
    }
}

pub fn benchmark_generating_walks(c: &mut Criterion) {
    let mut dp = DynamicProgramBuilder::new()
        .simple()
        .time_limit(600)
        .kernel(Kernel::from_generator(SimpleRwGenerator).unwrap())
        .build()
        .unwrap();

    dp.compute();

    let time_steps_list = (100..=500).step_by(50);
    let mut group = c.benchmark_group("DP generate walks");

    for time_steps in time_steps_list {
        group.sample_size(100).bench_with_input(
            BenchmarkId::new("single walk", time_steps),
            &time_steps,
            |b, &time_steps| {
                let walker = StandardWalker;

                b.iter(|| {
                    let _ = walker.generate_paths(&dp, 1, 50, 50, time_steps);
                });
            },
        );

        group.sample_size(10).bench_with_input(
            BenchmarkId::new("10 walks", time_steps),
            &time_steps,
            |b, &time_steps| {
                let walker = StandardWalker;

                b.iter(|| {
                    let _ = walker.generate_paths(&dp, 10, 50, 50, time_steps);
                });
            },
        );
    }
}

criterion_group!(
    benches,
    benchmark_dp_simple,
    benchmark_dp_correlated,
    benchmark_generating_walks
);
criterion_main!(benches);
