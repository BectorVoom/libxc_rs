//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1076/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1076<F: Float>(t40590: F, t68: F, t3700: F, t2751: F, t10047: F, t225: F, t9587: F, t9585: F, t10108: F, t257: F, t252: F, t9957: F) -> (F, F, F, F, F, F, F, F) {
    let t40591 = t68 * t40590;
    let t40610 = t3700 * t3700;
    let t40611 = F::new(1.0) / t40610;
    let t40771 = t2751 * t2751;
    let t40772 = F::new(1.0) / t40771;
    let t40852 = t10047 * t225;
    let t40870 = t9587 * t225;
    let t40875 = t9585 * t225;
    let t40889 = F::new(1.0) / t10108 / t257;
    let t40890 = t68 * t40889;
    let t40909 = t252 * t9957;
    (t40591, t40611, t40772, t40852, t40870, t40875, t40890, t40909)
}
