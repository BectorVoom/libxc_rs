//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2071/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2071<F: Float>(t10402: F, t11037: F, t2402: F, t973: F, t999: F, t1030: F, t10477: F, t10472: F, t10475: F, t3128: F, t10969: F, t121: F) -> (F, F, F, F, F, F) {
    let t42546 = t11037 * t10402;
    let t42552 = t973 * t2402 * t999;
    let t42559 = t1030 * t10477;
    let t42561 = t10472 * t10475 * t42559;
    let t42565 = t10472 * t3128 * t42559;
    let t42592 = t121 * t10969;
    (t42546, t42552, t42559, t42561, t42565, t42592)
}
