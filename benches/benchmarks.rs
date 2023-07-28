use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use randomwalks_lib::dp::simple::SimpleDynamicProgram;
use randomwalks_lib::dp::{DynamicProgramOptions, DynamicPrograms};
use randomwalks_lib::kernel::simple_rw::SimpleRwGenerator;
use randomwalks_lib::kernel::Kernel;

pub fn benchmark_dp(c: &mut Criterion) {
    let time_limits = (100..=1000).step_by(50);
    let mut group = c.benchmark_group("DP compute");

    for time_limit in time_limits {
        group.sample_size(10).bench_with_input(
            BenchmarkId::from_parameter(time_limit),
            &time_limit,
            |b, &time_limit| {
                let options = DynamicProgramOptions {
                    time_limit,
                    kernel: Some(Kernel::from_generator(SimpleRwGenerator {}).unwrap()),
                    ..Default::default()
                };

                let mut dp = SimpleDynamicProgram::new(options);

                b.iter(|| {
                    dp.compute();
                });
            },
        );
    }
}

criterion_group!(benches, benchmark_dp);
criterion_main!(benches);
