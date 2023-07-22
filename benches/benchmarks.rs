use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use randomwalks_lib::dp::simple::SimpleDynamicProgram;
use randomwalks_lib::dp::{DynamicProgram, DynamicProgramOptions};
use randomwalks_lib::kernel::simple_rw::SimpleRwGenerator;
use randomwalks_lib::kernel::Kernel;

pub fn benchmark_dp(c: &mut Criterion) {
    let options = DynamicProgramOptions {
        time_limit: 400,
        kernel: Some(Kernel::from_generator(SimpleRwGenerator {}).unwrap()),
        ..Default::default()
    };

    let mut dp = SimpleDynamicProgram::new(options);

    let time_limit = 400;

    c.bench_with_input(
        BenchmarkId::new("DP compute", time_limit),
        &time_limit,
        |b, &s| {
            b.iter(|| {
                dp.compute();
            });
        },
    );
}

criterion_group!(benches, benchmark_dp);
criterion_main!(benches);
