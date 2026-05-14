//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 693/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk693<F: Float>(t1971: F, t3351: F, t41015: F, t875: F, t3154: F, t38351: F, t1494: F, t1970: F, t209: F, t515: F, t664: F, t3352: F, t70423: F, t8456: F, t14225: F, t7248: F, t9170: F) -> (F, F, F, F, F) {
    let t74354 = t3351 * t1971 * t875 * t41015;
    let t74356 = t38351 * t3154;
    let t74368 = t1970 * t1971 * t515 * t664 * t1494 * t209;
    let t74371 = t70423 * t3352 * t8456;
    let t74374 = t14225 * t7248 * t9170;
    (t74354, t74356, t74368, t74371, t74374)
}
