//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 464/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk464<F: Float>(t14107: F, t899: F, t14020: F, t3113: F, t1179: F, t830: F, t14024: F) -> (F, F, F, F) {
    let t14108 = t899 * t14107;
    let t14113 = t3113 * t14020;
    let t14114 = t1179 * t830;
    let t14115 = t14114 * t14024;
    (t14108, t14113, t14114, t14115)
}
