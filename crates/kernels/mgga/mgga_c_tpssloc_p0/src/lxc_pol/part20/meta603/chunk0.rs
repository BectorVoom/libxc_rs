//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2183/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2183<F: Float>(t1174: F, t11765: F, t135: F, t3551: F, t698: F, t3242: F, t415: F, t42341: F, t44696: F, t42344: F, t483: F, t1210: F) -> (F, F, F, F, F, F) {
    let t44803 = t1174 * t135 * t11765;
    let t44811 = t1174 * t698 * t3551;
    let t44827 = F::new(1.0) / t415 / t3242;
    let t44833 = t44696 * t42341;
    let t44834 = t483 * t42344;
    let t44836 = t44833 * t1210 * t44834;
    (t44803, t44811, t44827, t44833, t44834, t44836)
}
