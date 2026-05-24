//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 898/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk898<F: Float>(t2415: F, t39553: F, t7349: F, t1525: F, t236: F, t3352: F, t551: F, t7230: F, t1587: F, t615: F, t10044: F, t1982: F, t7428: F) -> (F, F, F, F) {
    let t44894 = t7349 * t2415 * t39553;
    let t44901 = t7230 * t3352 * t236 * t551 * t1525;
    let t44906 = t7230 * t3352 * t236 * t1587 * t615;
    let t44909 = t10044 * t7428 * t1982;
    (t44894, t44901, t44906, t44909)
}
