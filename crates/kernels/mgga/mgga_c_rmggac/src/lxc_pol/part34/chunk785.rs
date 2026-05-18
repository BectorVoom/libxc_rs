//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 785/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk785<F: Float>(t14124: F, t21714: F, t236: F, t321: F, t598: F, t14125: F, t333: F, t511: F, t352: F, t515: F, t15367: F, t69568: F) -> (F, F, F, F) {
    let t74142 = t14124 * t21714 * t236 * t598 * t321;
    let t74147 = t14124 * t14125 * t511 * t598 * t333;
    let t74152 = t14124 * t14125 * t515 * t598 * t352;
    let t74154 = t69568 * t15367;
    (t74142, t74147, t74152, t74154)
}
