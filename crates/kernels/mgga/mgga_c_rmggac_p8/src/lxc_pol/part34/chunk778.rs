//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 778/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk778<F: Float>(t3140: F, t9221: F, t13868: F, t14230: F, t1525: F, t2067: F, t26: F, t3369: F, t15227: F, t68444: F, t68386: F, t7248: F, t9122: F) -> (F, F, F, F) {
    let t74035 = t9221 * t3140;
    let t74036 = t74035 * t13868;
    let t74041 = t14230 * t3369 * t2067 * t26 * t1525;
    let t74043 = t68444 * t15227;
    let t74046 = t68386 * t7248 * t9122;
    (t74036, t74041, t74043, t74046)
}
