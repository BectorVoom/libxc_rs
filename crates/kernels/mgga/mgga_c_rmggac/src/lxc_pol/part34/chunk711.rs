//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 711/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk711<F: Float>(t14125: F, t68440: F, t8835: F, t8842: F, t13962: F, t3056: F, t8486: F, t68946: F, t15397: F, t498: F, t14230: F, t14237: F, t2067: F, t321: F, t14243: F, t13828: F, t8368: F) -> (F, F, F, F, F, F, F) {
    let t74772 = t68440 * t14125 * t8835;
    let t74775 = t68440 * t14125 * t8842;
    let t74779 = t3056 * t13962 * t8486;
    let t74782 = 0.19863479950205658386e-4 * t68946;
    let t74783 = t15397 * t498;
    let t74786 = t14230 * t14237 * t2067 * t74783;
    let t74788 = t15397 * t321;
    let t74791 = t14230 * t14243 * t2067 * t74788;
    let t74793 = t8368 * t13828;
    (t74772, t74775, t74779, t74782, t74786, t74791, t74793)
}
