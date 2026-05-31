//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2961/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2961<F: Float>(t17884: F, t3048: F, t1046: F, t10962: F, t14085: F, t14093: F, t14491: F, t1618: F, t42570: F, t4636: F, t4641: F, t4644: F, t48430: F, t48441: F, t49866: F, t5869: F, t5875: F, t61695: F, t61699: F, t61705: F, t61708: F, t61710: F, t61713: F) -> F {
    let t61715 = t3048 * t17884;
    let t61717 = t14085 * t4636 / F::cast_from(1152.0_f64) + t4644 * t14093 / F::cast_from(2304.0_f64) + t49866 * t1618 / F::cast_from(1536.0_f64) + t10962 * t5869 / F::cast_from(3072.0_f64) - t61695 / F::cast_from(432.0_f64) + t48430 / F::cast_from(648.0_f64) + t61699 / F::cast_from(432.0_f64) + t4641 * t14491 / F::cast_from(1536.0_f64) - t42570 * t5875 / F::cast_from(144.0_f64) + t61705 / F::cast_from(1152.0_f64) - t48441 / F::cast_from(54.0_f64) + t61708 / F::cast_from(3456.0_f64) - t61710 * t1046 / F::cast_from(432.0_f64) + t61713 / F::cast_from(2304.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1944.0_f64) * t61715;
    t61717
}
