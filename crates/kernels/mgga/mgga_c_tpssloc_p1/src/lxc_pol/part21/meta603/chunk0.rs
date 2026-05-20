//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2360/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2360<F: Float>(t268: F, t521: F, t9799: F, t9847: F, t677: F, t9494: F, t3684: F, t12110: F, t9885: F, t12099: F, t2663: F, t2505: F, t2527: F) -> (F, F, F, F, F, F, F, F) {
    let t39321 = t521 * t268;
    let t39322 = t9799 * t9847;
    let t39324 = F::cast_from(0.1301229756036208781e0_f64) * t39321 * t39322;
    let t39325 = t677 * t9494;
    let t39327 = F::cast_from(0.38025319932552508021e2_f64) * t3684 * t39325;
    let t39328 = t12110 * t9885;
    let t39330 = t12099 * t2663;
    let t39336 = t2527 * t2505;
    (t39321, t39322, t39324, t39325, t39327, t39328, t39330, t39336)
}
