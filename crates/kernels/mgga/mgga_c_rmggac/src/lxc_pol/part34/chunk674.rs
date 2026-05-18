//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 674/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk674<F: Float>(t326: F, t7417: F, t1179: F, t14024: F, t3899: F, t14113: F, t14118: F, t2144: F, t2529: F, t1971: F, t3121: F, t14114: F) -> (F, F, F, F, F, F) {
    let t68471 = t7417 * t326;
    let t68489 = t1179 * t3899 * t14024;
    let t68490 = t14113 * t68489;
    let t68491 = t68490 * t14118;
    let t68498 = t2144 * t2529;
    let t68499 = t1971 * t68498;
    let t68502 = t14024 * t3121;
    let t68503 = t14114 * t68502;
    (t68471, t68489, t68490, t68491, t68499, t68503)
}
