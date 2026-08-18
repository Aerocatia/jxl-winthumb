use jxl::api::{JxlParallelRunner, JxlParallelRunnerFun};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct RayonParallelRunner;

impl JxlParallelRunner for RayonParallelRunner {
    fn run(&mut self, num: usize, fun: &JxlParallelRunnerFun) -> jxl::error::Result<()> {
        if num == 1 || rayon::current_num_threads() == 1 {
            for i in 0..num {
                fun(i)?;
            }
            return Ok(());
        }
        (0..num).into_par_iter().try_for_each(fun)
    }
}
