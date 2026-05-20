//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1234/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1234<F: Float>(t2617: F, t9973: F, t236: F, t40931: F, t240: F, t812: F, t2638: F, t9612: F, t831: F, t10021: F, t815: F, t2686: F, t9671: F) -> (F, F, F, F, F) {
    let t41344 = t2617 * t9973;
    let t41347 = t40931 * t236;
    let t41349 = t812 * t41347 * t240;
    let t41354 = t9612 * t2638;
    let t41355 = t41354 * t831;
    let t41362 = t812 * t815 * t10021;
    let t41363 = t41362 * t831;
    let t41365 = t9671 * t2686;
    (t41344, t41349, t41355, t41363, t41365)
}
