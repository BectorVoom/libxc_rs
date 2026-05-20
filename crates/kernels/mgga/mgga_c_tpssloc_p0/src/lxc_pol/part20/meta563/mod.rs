//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2121;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2122;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta563<F: Float>(t10403: F, t10404: F, t10422: F, t10477: F, t67: F, t3067: F, t11059: F, t10970: F, t820: F, t10418: F, t3070: F, t10397: F, t10517: F, t3103: F, t1041: F, t10868: F, t248: F, t2780: F, t10316: F, t3051: F, t10277: F, t976: F, t10993: F, t2960: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t42380, t42386, t42387, t42388, t42397, t42403, t42412) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2121::<F>(t10403, t10404, t10422, t10477, t67, t3067, t11059, t10970, t820, t10418, t3070, t10397);
        let (t42428, t42432, t42436, t42444, t42460) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2122::<F>(t10517, t3103, t1041, t10868, t248, t2780, t10316, t3051, t10277, t976, t10993, t2960);
    (t42380, t42386, t42387, t42388, t42397, t42403, t42412, t42428, t42432, t42436, t42444, t42460)
}
