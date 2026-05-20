//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2125/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2125<F: Float>(t10987: F, t135: F, t973: F, t10402: F, t11034: F, t11037: F, t2402: F, t999: F, t9277: F, t972: F, t10263: F, t3139: F) -> (F, F, F, F, F, F) {
    let t42530 = t973 * t135 * t10987;
    let t42541 = t11034 * t10402;
    let t42546 = t11037 * t10402;
    let t42552 = t973 * t2402 * t999;
    let t42554 = t9277 * t972;
    let t42557 = t10263 * t3139;
    (t42530, t42541, t42546, t42552, t42554, t42557)
}
