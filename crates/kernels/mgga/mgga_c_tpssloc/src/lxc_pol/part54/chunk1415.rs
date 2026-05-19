//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1415/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1415<F: Float>(t1985: F, t214: F, t225: F, t27051: F, t567: F, t22666: F, t33296: F, t7918: F, t6907: F, t33259: F, t115354: F, t120232: F, t120239: F, t120244: F, t120247: F, t120253: F, t120258: F, t1386: F, t2016: F, t2092: F, t26366: F, t7214: F, t91441: F, t93316: F) -> (F, F) {
    let t122160 = t1985 * t214 * t27051 * t225 * t567;
    let t122164 = t1985 * t22666 * t33296;
    let t122166 = t214 * t7918;
    let t122168 = t1985 * t122166 * t6907;
    let t122172 = t33259 * t225;
    let t122174 = F::cast_from(0.82246703342411321824e-2_f64) * t115354 + F::cast_from(0.82246703342411321825e-2_f64) * t122160 - t120232 - t91441 * t2092 - F::cast_from(0.82246703342411321825e-2_f64) * t122164 - F::cast_from(0.82246703342411321825e-2_f64) * t122168 - t120239 - t120244 - t26366 * t7214 + t120247 + t120253 - t93316 * t2016 - t122172 * t1386 + t120258;
    (t122166, t122174)
}
