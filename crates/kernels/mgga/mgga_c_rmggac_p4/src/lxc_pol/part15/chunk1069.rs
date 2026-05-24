//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1069/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1069<F: Float>(t39277: F, t9046: F, t17859: F, t8812: F, t2289: F, t38472: F, t46075: F, t903: F, t2185: F, t678: F, t9825: F, t535: F, t577: F, t7933: F, t7934: F) -> (F, F, F, F, F, F) {
    let t47532 = t39277 * t9046;
    let t47534 = t17859 * t8812;
    let t47536 = t38472 * t2289;
    let t47538 = t903 * t46075;
    let t47541 = t9825 * t2185 * t678;
    let t47545 = t7933 * t7934 * t577 * t535;
    (t47532, t47534, t47536, t47538, t47541, t47545)
}
