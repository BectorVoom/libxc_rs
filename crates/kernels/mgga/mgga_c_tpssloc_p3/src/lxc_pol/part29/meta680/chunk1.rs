//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2287/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2287<F: Float>(t27438: F, t85639: F, t225: F, t27419: F, t1236: F, t1252: F, t12652: F, t1409: F, t15797: F, t15820: F, t2128: F, t24589: F, t24590: F, t24601: F, t24602: F, t24626: F, t24638: F, t24877: F, t254: F, t27388: F, t27406: F, t27444: F, t27747: F, t27786: F, t3487: F, t3630: F, t4936: F, t4945: F, t7356: F, t7392: F) -> F {
    let t94648 = F::cast_from(0.18277045187202515961e-2_f64) * t85639 * t27438;
    let t94656 = t27419 * t225;
    let t94673 = -F::new(2.0) * t15797 * t7392 + F::new(4.0) * t3487 * t27747 - F::new(2.0) * t15820 * t7392 - F::new(12.0) * t1236 * t254 * t27786 + t94648 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t24590 * t27388 - F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t24601 * t27444 * t12652 - F::new(2.0) * t94656 * t1252 + F::new(4.0) * t15797 * t7356 + F::new(2.0) * t4945 * t24877 + F::cast_from(0.16449340668482264365e-1_f64) * t2128 * t4936 * t24638 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t24601 * t24602 * t1409 * t3630 + F::cast_from(0.21932454224643019153e-1_f64) * t27406 * t24626;
    t94673
}
