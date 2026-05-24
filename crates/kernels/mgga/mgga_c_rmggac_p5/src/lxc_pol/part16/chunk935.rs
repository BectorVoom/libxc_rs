//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 935/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk935<F: Float>(t1763: F, t1971: F, t495: F, t7230: F, t875: F, t3351: F, t498: F, t7231: F, t30800: F, t3352: F, t30490: F, t7262: F) -> (F, F, F, F) {
    let t45541 = t7230 * t1971 * t875 * t1763 * t495;
    let t45546 = t3351 * t7231 * t875 * t1763 * t498;
    let t45550 = t3351 * t3352 * t875 * t30800;
    let t45554 = t3351 * t1971 * t7262 * t30490;
    (t45541, t45546, t45550, t45554)
}
