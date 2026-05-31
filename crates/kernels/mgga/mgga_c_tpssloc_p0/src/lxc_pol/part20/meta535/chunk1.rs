//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2074/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2074<F: Float>(t2517: F, t2519: F, t195: F, t632: F, t197: F, t636: F, t2531: F, t9892: F, t67: F, t758: F, t9915: F, t718: F, t9862: F) -> (F, F, F, F, F, F) {
    let t40626 = t2519 * t2517;
    let t40632 = F::cast_from(1.0_f64) / t195 / t632;
    let t40647 = F::cast_from(1.0_f64) / t197 / t636;
    let t40667 = t2531 * t9892;
    let t40670 = t9915 * t67 * t758;
    let t40673 = t718 * t9862;
    (t40626, t40632, t40647, t40667, t40670, t40673)
}
