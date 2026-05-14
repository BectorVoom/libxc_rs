//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 922/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk922<F: Float>(t221: F, t4128: F, t5544: F, t20800: F, t210: F, t214: F, t12986: F, t13010: F, t13022: F, t16769: F, t16784: F, t16792: F, t16794: F, t20923: F, t4127: F, t787: F, t9540: F, t9559: F, t9572: F, t9579: F, t9583: F) -> (F, F, F) {
    let t20927 = t221 * t4128 * t5544;
    let t20933 = t210 * t214 * t20800;
    let t20936 = -t9540 + 0.49999999999999999998e-2 * t12986 - t9572 - 0.34999999999999999998e-1 * t16769 - 0.38888888888888888888e-1 * t13010 - 0.74999999999999999997e-2 * t16784 + 0.24999999999999999999e-2 * t16792 - 0.19999999999999999999e-1 * t9559 * t20923 + 0.14999999999999999999e-1 * t4127 * t20927 + t9579 + 0.11666666666666666666e-1 * t16794 - 0.15833333333333333333e-1 * t13022 - 0.16666666666666666666e-2 * t787 * t20933 - t9583;
    (t20927, t20933, t20936)
}
