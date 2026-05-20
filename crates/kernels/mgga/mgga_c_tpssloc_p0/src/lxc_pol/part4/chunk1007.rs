//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1007/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1007<F: Float>(t16796: F, t221: F, t776: F, t13014: F, t13020: F, t13022: F, t13027: F, t16784: F, t16787: F, t16792: F, t16794: F, t4127: F, t787: F, t9579: F, t9583: F) -> F {
    let t16798 = t221 * t16796 * t776;
    let t16803 = -t13014 - F::cast_from(0.24999999999999999999e-2_f64) * t16784 - F::cast_from(0.16666666666666666666e-2_f64) * t787 * t16787 + F::cast_from(0.8333333333333333333e-3_f64) * t16792 + t9579 + F::cast_from(0.38888888888888888887e-2_f64) * t16794 + F::cast_from(0.49999999999999999998e-2_f64) * t4127 * t16798 + F::cast_from(0.77777777777777777775e-2_f64) * t13020 - F::cast_from(0.10555555555555555555e-1_f64) * t13022 + t13027 - t9583;
    t16803
}
