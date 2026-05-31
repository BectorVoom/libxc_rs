//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1121/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1121<F: Float>(t23066: F, t6591: F, t229: F, t6546: F, t805: F, t2628: F, t2633: F, t6605: F, t243: F, t598: F, t213: F, t1894: F, t236: F, t2379: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t23067 = t6591 * t23066;
    let t23069 = t6546 * t229;
    let t23070 = t23069 * t805;
    let t23071 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t23070;
    let t23072 = t2628 * t2633;
    let t23073 = t6605 * t23072;
    let t23075 = t243 * t243;
    let t23076 = F::cast_from(1.0_f64) / t23075;
    let t23077 = t598 * t23076;
    let t23078 = t23077 * t213;
    let t23080 = t1894 * t236 * t2379;
    (t23067, t23069, t23071, t23072, t23073, t23075, t23076, t23077, t23078, t23080)
}
