//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 548/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk548<F: Float>(t1229: F, t3247: F, t3961: F, t4582: F, t1734: F, t486: F, t1215: F, t3508: F, t1216: F, t3242: F, t3584: F, t1653: F, t248: F, t3521: F) -> (F, F, F, F, F) {
    let t4972 = t1229 * t3247;
    let t4973 = t4972 * t3961;
    let t4974 = t4582 * t4973;
    let t4977 = t486 * t1734;
    let t4978 = t3508 * t1215;
    let t4979 = t4977 * t4978;
    let t4980 = t4582 * t4979;
    let t4983 = t4977 * t1216;
    let t4984 = t4582 * t4983;
    let t4987 = t3584 * t3242;
    let t4988 = t4987 * t3961;
    let t4989 = t4582 * t4988;
    let t4993 = t248 * t3521 * t1653;
    (t4974, t4980, t4984, t4989, t4993)
}
