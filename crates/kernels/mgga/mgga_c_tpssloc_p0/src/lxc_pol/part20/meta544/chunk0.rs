//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2085/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2085<F: Float>(t10033: F, t41011: F, t2632: F, t9957: F, t9638: F, t9653: F, t9623: F, t10003: F, t10009: F, t2617: F, t9600: F, t849: F) -> (F, F, F, F, F, F, F, F) {
    let t41012 = t41011 * t10033;
    let t41014 = t2632 * t9957;
    let t41025 = t9638 * t9653;
    let t41031 = t9638 * t9623;
    let t41048 = t9638 * t10003;
    let t41050 = t9638 * t10009;
    let t41052 = t2617 * t9600;
    let t41053 = t41052 * t849;
    (t41012, t41014, t41025, t41031, t41048, t41050, t41052, t41053)
}
