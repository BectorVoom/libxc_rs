//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1090/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1090<F: Float>(t321: F, t3352: F, t511: F, t7230: F, t9969: F, t45347: F, t674: F, t2004: F, t2007: F, t1971: F, t2144: F, t30311: F, t3351: F) -> (F, F, F, F, F) {
    let t47845 = t7230 * t3352 * t511 * t9969 * t321;
    let t47854 = t45347 * t674;
    let t47855 = t47854 * t2004;
    let t47857 = t47854 * t2007;
    let t47861 = t3351 * t1971 * t2144 * t30311;
    (t47845, t47854, t47855, t47857, t47861)
}
