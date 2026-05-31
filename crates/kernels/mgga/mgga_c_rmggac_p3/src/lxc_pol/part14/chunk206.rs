//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 206/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk206<F: Float>(t50: F, t814: F, t278: F, t90: F, t100: F, t316: F, t101: F, t34: F, t77: F) -> (F, F, F, F, F, F, F, F, F) {
    let t815 = t50 * t814;
    let t816 = -t278 + t815;
    let t817 = t90 * t816;
    let t820 = F::cast_from(1.0_f64) / t100;
    let t821 = t316 * t316;
    let t822 = t820 * t821;
    let t825 = -t816;
    let t826 = t101 * t825;
    let t830 = F::cast_from(1.0_f64) / t34 / t77;
    (t815, t816, t817, t820, t821, t822, t825, t826, t830)
}
