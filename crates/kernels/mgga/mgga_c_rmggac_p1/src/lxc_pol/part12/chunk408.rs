//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 408/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk408<F: Float>(t88: F, t89: F, t154: F, t50: F, t100: F, t99: F, t297: F, t34: F) -> (F, F, F, F) {
    let t3868 = t89 * t88;
    let t3869 = F::cast_from(1.0_f64) / t3868;
    let t3878 = t50 * t154;
    let t3884 = t100 * t99;
    let t3885 = F::cast_from(1.0_f64) / t3884;
    let t3899 = F::cast_from(1.0_f64) / t34 / t297;
    (t3869, t3878, t3885, t3899)
}
