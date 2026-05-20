//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2142/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2142<F: Float>(t11292: F, t1687: F, t50826: F, t50948: F, t11365: F, t1694: F, t3331: F, t4794: F, t50919: F, t300: F, t3401: F, t11310: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t51680 = t1687 * t11292;
    let t51683 = F::cast_from(0.12361111111111111111e-1_f64) * t50826;
    let t51707 = F::cast_from(0.24722222222222222222e-1_f64) * t50948;
    let t51727 = t11365 * t1694;
    let t51730 = t4794 * t3331;
    let t51745 = F::cast_from(0.2283111111111111111e-1_f64) * t50826;
    let t51760 = F::cast_from(0.1522074074074074074e-1_f64) * t50919;
    let t51769 = F::cast_from(0.4566222222222222222e-1_f64) * t50948;
    let t51810 = t300 * t3401;
    let t51819 = t300 * t11310;
    (t51680, t51683, t51707, t51727, t51730, t51745, t51760, t51769, t51810, t51819)
}
