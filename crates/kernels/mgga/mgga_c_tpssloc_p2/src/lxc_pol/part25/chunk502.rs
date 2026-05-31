//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 502/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk502<F: Float>(t2836: F, t913: F, t893: F, t891: F, t275: F, t290: F, t2793: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2837 = t2836 * t913;
    let t2839 = F::cast_from(1.0_f64) * t893 * t2837;
    let t2840 = t891 * t891;
    let t2841 = F::cast_from(1.0_f64) / t2840;
    let t2842 = t275 * t2841;
    let t2843 = t290 * t290;
    let t2844 = F::cast_from(1.0_f64) / t2843;
    let t2845 = t2793 * t2844;
    let t2847 = F::cast_from(0.16081979498692535067e2_f64) * t2842 * t2845;
    let t2848 = F::cast_from(0.22831111111111111111e-1_f64) * t2764;
    let t2853 = t2848 + F::cast_from(0.11415555555555555555e-1_f64) * t2766 - F::cast_from(0.11415555555555555555e-1_f64) * t2773 + F::cast_from(0.34246666666666666666e-1_f64) * t2778 - F::cast_from(0.17123333333333333333e-1_f64) * t2782;
    (t2837, t2839, t2840, t2841, t2842, t2843, t2844, t2845, t2847, t2853)
}
