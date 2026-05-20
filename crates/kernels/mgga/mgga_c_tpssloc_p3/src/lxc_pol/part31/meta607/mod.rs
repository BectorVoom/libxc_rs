//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta607<F: Float>(t90549: F, t90584: F, t90604: F, t90609: F, t90645: F, t90686: F, t90701: F, t90707: F, t90749: F, t90759: F, t90781: F, t90789: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t93362, t93388, t93404, t93407, t93439, t93452, t93461, t93467, t93473, t93476, t93483, t93488) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1852::<F>(t90549, t90584, t90604, t90609, t90645, t90686, t90701, t90707, t90749, t90759, t90781, t90789);
    (t93362, t93388, t93404, t93407, t93439, t93452, t93461, t93467, t93473, t93476, t93483, t93488)
}
