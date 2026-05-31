//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1976/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1976<F: Float>(t12328: F, t2003: F, t12248: F, t59: F, t1336: F, t2690: F, t6943: F, t1354: F, t22865: F, t6604: F, t6937: F, t22811: F, t61: F) -> (F, F, F, F, F, F, F) {
    let t80899 = t2003 * t12328;
    let t80900 = F::cast_from(595.0_f64) / F::cast_from(5184.0_f64) * t80899;
    let t80901 = t12248 * t59;
    let t80914 = t1336 * t6943 * t2690;
    let t80915 = t80914 * t1354;
    let t80939 = t22865 * t6604;
    let t80940 = t80939 * t6937;
    let t80953 = F::cast_from(1.0_f64) / t61 / t22811;
    (t80900, t80901, t80914, t80915, t80939, t80940, t80953)
}
