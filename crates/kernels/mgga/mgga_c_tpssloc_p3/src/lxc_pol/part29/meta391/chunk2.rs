//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1600/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1600<F: Float>(t14781: F, t11137: F, t11139: F, t11141: F, t11143: F, t14728: F, t14809: F, t14811: F, t14814: F, t14816: F, t14818: F, t14824: F) -> (F, F) {
    let t14890 = F::cast_from(0.21908444444444444444e0_f64) * t14781;
    let t14911 = -F::new(0.1898925e1) * t14809 - F::new(0.9494625e0) * t14811 + F::new(0.3071625e0) * t14814 + F::new(0.15358125e0) * t14816 + F::cast_from(0.36514074074074074074e-1_f64) * t14818 + F::cast_from(0.26574814814814814816e0_f64) * t11137 + F::cast_from(0.66437037037037037038e-1_f64) * t11139 - F::cast_from(0.19931111111111111111e0_f64) * t11141 - F::cast_from(0.99655555555555555557e-1_f64) * t11143 + F::new(0.3071625e0) * t14824 + F::cast_from(0.33218518518518518518e0_f64) * t14728;
    (t14890, t14911)
}
