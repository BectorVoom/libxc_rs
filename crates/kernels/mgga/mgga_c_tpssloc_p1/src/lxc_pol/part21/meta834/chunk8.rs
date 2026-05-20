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
    let t61717 = t14085 * t4636 / F::new(1152.0) + t4644 * t14093 / F::new(2304.0) + t49866 * t1618 / F::new(1536.0) + t10962 * t5869 / F::new(3072.0) - t61695 / F::new(432.0) + t48430 / F::new(648.0) + t61699 / F::new(432.0) + t4641 * t14491 / F::new(1536.0) - t42570 * t5875 / F::new(144.0) + t61705 / F::new(1152.0) - t48441 / F::new(54.0) + t61708 / F::new(3456.0) - t61710 * t1046 / F::new(432.0) + t61713 / F::new(2304.0) - F::new(5.0) / F::new(1944.0) * t61715;
    t61717
}
