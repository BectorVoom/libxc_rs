//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 923/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk923<F: Float>(t76700: F, t15450: F, t7244: F, t1971: F, t495: F, t7230: F, t875: F, t9551: F, t15626: F, t34884: F, t3352: F, t515: F, t9523: F) -> (F, F, F, F, F) {
    let t76701 = F::new(0.25538759935978703639e-4) * t76700;
    let t76702 = t7244 * t15450;
    let t76703 = F::new(0.99317399751028291929e-5) * t76702;
    let t76707 = t7230 * t1971 * t875 * t9551 * t495;
    let t76708 = F::new(0.1064114997332445985e-4) * t76707;
    let t76712 = t34884 * t15626;
    let t76713 = F::new(0.12414674968878536491e-4) * t76712;
    let t76717 = t7230 * t3352 * t515 * t9523 * t495;
    (t76701, t76703, t76708, t76713, t76717)
}
