//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1190/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1190<F: Float>(t52: F, t197: F, t636: F, t2244: F, t2250: F, t2440: F, t39097: F, t39103: F, t39110: F, t76: F, t9258: F, t9438: F, t9441: F, zeta_threshold: F) -> F {
    let t150 = t52 <= zeta_threshold;
    let t40647 = F::cast_from(1.0_f64) / t197 / t636;
    let t40660 = piecewise3::<F>(t150, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t40647 * t39097 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t9438 * t2244 * t2250 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2440 * t39103 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t9441 * t9258 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t76 * t39110);
    t40660
}
