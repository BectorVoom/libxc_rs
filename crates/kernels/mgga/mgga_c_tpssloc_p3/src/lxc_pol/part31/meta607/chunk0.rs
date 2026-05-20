//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1852/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1852<F: Float>(t90549: F, t90584: F, t90604: F, t90609: F, t90645: F, t90686: F, t90701: F, t90707: F, t90749: F, t90759: F, t90781: F, t90789: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93362 = F::cast_from(0.3289868133696452873e-1_f64) * t90549;
    let t93388 = F::cast_from(0.15352717957250113407e0_f64) * t90584;
    let t93404 = F::cast_from(0.76763589786250567036e-1_f64) * t90604;
    let t93407 = F::cast_from(0.9869604401089358619e-1_f64) * t90609;
    let t93439 = F::cast_from(0.16449340668482264365e-1_f64) * t90645;
    let t93452 = F::cast_from(0.3289868133696452873e-1_f64) * t90686;
    let t93461 = F::cast_from(0.16449340668482264365e-1_f64) * t90701;
    let t93467 = F::cast_from(0.76763589786250567036e-1_f64) * t90707;
    let t93473 = F::cast_from(0.15352717957250113407e0_f64) * t90749;
    let t93476 = F::cast_from(0.76763589786250567036e-1_f64) * t90759;
    let t93483 = F::cast_from(0.16449340668482264365e-1_f64) * t90781;
    let t93488 = F::cast_from(0.9869604401089358619e-1_f64) * t90789;
    (t93362, t93388, t93404, t93407, t93439, t93452, t93461, t93467, t93473, t93476, t93483, t93488)
}
