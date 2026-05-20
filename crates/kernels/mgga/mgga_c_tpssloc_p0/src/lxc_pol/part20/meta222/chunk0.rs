//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1295/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1295<F: Float>(t2239: F, t601: F, t83: F, t84: F, t85: F, t24: F, t2241: F, t645: F, t2307: F, t607: F, t65: F, t67: F) -> (F, F, F, F, F, F) {
    let t9231 = t601 * t2239;
    let t9238 = F::new(1.0) / t85 / t84 / t83;
    let t9239 = t24 * t9238;
    let t9240 = t2241 * t645;
    let t9243 = t645 * t2307;
    let t9247 = t607 * t65 * t67;
    (t9231, t9238, t9239, t9240, t9243, t9247)
}
