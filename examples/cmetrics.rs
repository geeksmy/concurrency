use anyhow::Result;
use concurrency::CmapMetrics;
use rand::Rng;
use std::thread;
use std::time::Duration;
use thread::spawn;

const M: usize = 4;
const N: usize = 4;

fn main() -> Result<()> {
    let metrics = CmapMetrics::new();

    println!("{}", metrics);

    for idx in 0..N {
        task_worker(idx, metrics.clone())?;
    }

    for _ in 0..M {
        request_worker(metrics.clone())?;
    }

    loop {
        thread::sleep(Duration::from_secs(2));
        println!("{}", metrics);
    }
}

fn task_worker(idx: usize, mut metrics: CmapMetrics) -> Result<()> {
    spawn(move || {
        loop {
            let rng = &mut rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(100..500)));
            metrics.inc(format!("call.thread.worker.{}", idx))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}

fn request_worker(mut metrics: CmapMetrics) -> Result<()> {
    spawn(move || {
        loop {
            let rng = &mut rand::thread_rng();

            thread::sleep(Duration::from_millis(rng.gen_range(50..800)));
            let page = rng.gen_range(1..5);
            metrics.inc(format!("req.page.{}", page))?;
        }
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    Ok(())
}
