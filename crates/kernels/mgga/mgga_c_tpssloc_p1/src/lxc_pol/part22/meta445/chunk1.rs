//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1797/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1797<F: Float>(t3726: F, t6358: F, t213: F, t6347: F, t1307: F, t221: F, t12228: F, t12236: F, t16078: F, t16083: F, t16099: F, t16106: F, t16108: F, t16113: F, t16119: F, t5195: F) -> (F, F, F) {
    let t19791 = t3726 * t6358;
    let t19793 = t213 * t6347;
    let t19795 = t221 * t19793 * t1307;
    let t19803 = F::cast_from(0.38888888888888888887e-2_f64) * t19791 + F::cast_from(0.49999999999999999998e-2_f64) * t5195 * t19795 + F::cast_from(0.16666666666666666666e-2_f64) * t12228 - F::cast_from(0.25925925925925925925e-1_f64) * t16078 - t16083 - t16099 - t12236 + F::cast_from(0.77777777777777777775e-2_f64) * t16106 - F::cast_from(0.10555555555555555555e-1_f64) * t16108 + t16113 + F::cast_from(0.33333333333333333332e-2_f64) * t16119;
    (t19791, t19795, t19803)
}
