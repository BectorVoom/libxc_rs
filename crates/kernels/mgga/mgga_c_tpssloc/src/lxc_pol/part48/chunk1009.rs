//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1009/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1009<F: Float>(t115723: F, t2039: F, t31537: F, t7056: F, t22479: F, t88: F, t31717: F, t23917: F, t8601: F, t114552: F, t1873: F, t84097: F) -> (F, F, F, F, F, F, F) {
    let t115783 = F::new(4.0) * t115723 * t2039;
    let t115785 = F::new(4.0) * t31537 * t7056;
    let t115786 = t88 * t22479;
    let t115788 = F::new(2.0) * t115786 * t2039;
    let t115790 = F::new(4.0) * t31717 * t7056;
    let t115792 = F::new(2.0) * t8601 * t23917;
    let t115796 = F::new(2.0) * t114552 * t2039;
    let t115802 = F::new(2.0) * t84097 * t1873;
    (t115783, t115785, t115788, t115790, t115792, t115796, t115802)
}
