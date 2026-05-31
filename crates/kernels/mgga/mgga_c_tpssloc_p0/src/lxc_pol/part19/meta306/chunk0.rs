//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1093/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1093<F: Float>(t601: F, t9238: F, t85: F, t24: F, t2241: F, t2307: F, t10276: F, t73: F, t2244: F) -> (F, F, F, F, F, F) {
    let t39054 = t601 * t9238;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t39064 = t2241 * t2241;
    let t39070 = t2307 * t2307;
    let t39096 = F::cast_from(1.0_f64) / t73 / t10276;
    let t39097 = t2244 * t2244;
    (t39054, t39063, t39064, t39070, t39096, t39097)
}
