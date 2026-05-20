//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta27 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk191;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk192;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk193;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk194;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk195;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk196;
use chunk6::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk197;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta27<F: Float>(t184: F, t521: F, t25: F, t28: F, t17: F, t182: F, t514: F, t194: F, t517: F, zeta_threshold: F, t154: F, t205: F, t215: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t522 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk191::<F>(t184, t521);
        let (t523, t525, t526, t528, t531) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk192::<F>(t25, t28, t17, t522, t182, t521, t514, t194, t517, zeta_threshold);
        let t532 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk193::<F>(t531);
        let t533 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk194::<F>(t531, t532);
        let t534 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk195::<F>(t532);
        let t535 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk196::<F>(t154, t534);
        let t539 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk197::<F>(t205, t215, t535);
    (t522, t523, t525, t526, t528, t531, t532, t533, t534, t535, t539)
}
