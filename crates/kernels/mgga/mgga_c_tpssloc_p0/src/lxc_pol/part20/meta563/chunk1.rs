//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2122/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2122<F: Float>(t10517: F, t3103: F, t1041: F, t10868: F, t248: F, t2780: F, t10316: F, t3051: F, t10277: F, t976: F, t10993: F, t2960: F) -> (F, F, F, F, F) {
    let t42428 = t10517 * t3103;
    let t42432 = t1041 * t248 * t10868 * t2780;
    let t42436 = t1041 * t248 * t3051 * t10316;
    let t42444 = t976 * t10277;
    let t42460 = t2960 * t10993;
    (t42428, t42432, t42436, t42444, t42460)
}
