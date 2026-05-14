//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 615/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk615<F: Float>(t14173: F, t3814: F, t35589: F, t664: F, t305: F, t3851: F, t68737: F, t3046: F, t874: F, t2044: F, t25636: F, t2048: F, t3826: F, t328: F, t3810: F, t2566: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t69176 = t3814 * t14173;
    let t69179 = t35589 * t664;
    let t69181 = 0.2927036860455597649e0 * t305 * t69179;
    let t69182 = t3851 * t68737;
    let t69183 = 0.23948483403727617128e0 * t69182;
    let t69184 = t874 * t3046;
    let t69195 = t25636 * t2044;
    let t69199 = t3826 * t2048;
    let t69200 = t69199 * t328;
    let t69201 = 0.2419210303588817044e-2 * t69200;
    let t69205 = t3810 * t2048;
    let t69206 = t69205 * t2566;
    (t69176, t69179, t69181, t69183, t69184, t69195, t69199, t69201, t69205, t69206)
}
