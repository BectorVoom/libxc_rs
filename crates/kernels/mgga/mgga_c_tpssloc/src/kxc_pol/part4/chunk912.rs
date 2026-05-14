//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 912/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk912<F: Float>(t16783: F, t9549: F, t16662: F, t210: F, t214: F, t118: F, t5544: F, t794: F, t2576: F, t2563: F, t5555: F, t213: F, t221: F, t776: F, t13014: F, t13020: F, t13022: F, t13027: F, t4127: F, t787: F, t9579: F, t9583: F) -> (F,) {
    let t16784 = t9549 * t16783;
    let t16787 = t210 * t214 * t16662;
    let t16791 = t118 * t794 * t5544;
    let t16792 = t2576 * t16791;
    let t16794 = t2563 * t5555;
    let t16796 = t213 * t5544;
    let t16798 = t221 * t16796 * t776;
    let t16803 = -t13014 - 0.24999999999999999999e-2 * t16784 - 0.16666666666666666666e-2 * t787 * t16787 + 0.8333333333333333333e-3 * t16792 + t9579 + 0.38888888888888888887e-2 * t16794 + 0.49999999999999999998e-2 * t4127 * t16798 + 0.77777777777777777775e-2 * t13020 - 0.10555555555555555555e-1 * t13022 + t13027 - t9583;
    (t16803,)
}
