//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1027/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1027<F: Float>(t25510: F, t25512: F, t1625: F, t362: F, t884: F, t6784: F, t6743: F, t7577: F, t6801: F, t1058: F, t23327: F, t23601: F, t23642: F, t23670: F, t25487: F, t25493: F, t25497: F, t25500: F, t25503: F, t25508: F, t3180: F, t6687: F, t6797: F, t7611: F, t7620: F) -> F {
    let t25513 = t25510 * t25512;
    let t25516 = t362 * t1625;
    let t25517 = t25516 * t884;
    let t25518 = t6784 * t25517;
    let t25523 = t7577 * t6743;
    let t25524 = t25523 * t6801;
    let t25527 = F::cast_from(0.16449340668482264365e-1_f64) * t23601 * t25487 - F::cast_from(0.82246703342411321825e-2_f64) * t23601 * t25493 + t1058 * t25497 + t1058 * t25500 + F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t25503 - F::cast_from(0.21932454224643019153e-1_f64) * t23670 * t7611 + F::cast_from(0.27415567780803773942e-2_f64) * t25508 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t25513 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t25518 - F::cast_from(0.27415567780803773942e-2_f64) * t23642 + t3180 * t7620 - F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t25524;
    t25527
}
