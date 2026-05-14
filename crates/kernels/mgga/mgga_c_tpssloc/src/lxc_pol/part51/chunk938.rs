//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 938/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk938<F: Float>(t23665: F, t7611: F, t1936: F, t362: F, t2775: F, t381: F, t3961: F, t1625: F, t884: F, t6784: F, t6743: F, t7577: F, t6801: F, t1058: F, t23327: F, t23601: F, t23642: F, t23670: F, t25487: F, t25493: F, t25497: F, t25500: F, t25503: F, t3180: F, t6687: F, t6797: F, t7620: F) -> (F, F) {
    let t25508 = t23665 * t7611;
    let t25510 = t1936 * t362;
    let t25511 = t381 * t2775;
    let t25512 = t25511 * t3961;
    let t25513 = t25510 * t25512;
    let t25516 = t362 * t1625;
    let t25517 = t25516 * t884;
    let t25518 = t6784 * t25517;
    let t25523 = t7577 * t6743;
    let t25524 = t25523 * t6801;
    let t25527 = 0.16449340668482264365e-1 * t23601 * t25487 - 0.82246703342411321825e-2 * t23601 * t25493 + t1058 * t25497 + t1058 * t25500 + 0.82246703342411321825e-2 * t6797 * t25503 - 0.21932454224643019153e-1 * t23670 * t7611 + 0.27415567780803773942e-2 * t25508 - 0.54831135561607547884e-2 * t23327 * t25513 + 0.27415567780803773942e-2 * t6687 * t25518 - 0.27415567780803773942e-2 * t23642 + t3180 * t7620 - 0.82246703342411321825e-2 * t6797 * t25524;
    (t25510, t25527)
}
