//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 912/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk912<F: Float>(t236: F, t3351: F, t5204: F, t9188: F, t3352: F, t511: F, t5211: F, t2004: F, t38472: F, t2320: F, t36315: F, t7717: F, t8616: F) -> (F, F, F, F, F) {
    let t39748 = t3351 * t9188 * t236 * t5204;
    let t39752 = t3351 * t3352 * t511 * t5211;
    let t39754 = t38472 * t2004;
    let t39756 = t36315 * t2320;
    let t39758 = t7717 * t8616;
    (t39748, t39752, t39754, t39756, t39758)
}
