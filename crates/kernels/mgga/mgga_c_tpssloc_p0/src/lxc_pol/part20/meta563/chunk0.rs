//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2121/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2121<F: Float>(t10403: F, t10404: F, t10422: F, t10477: F, t67: F, t3067: F, t11059: F, t10970: F, t820: F, t10418: F, t3070: F, t10397: F) -> (F, F, F, F, F, F, F) {
    let t42380 = t10403 * t10422 * t10404;
    let t42386 = t10477 * t67;
    let t42387 = t3067 * t42386;
    let t42388 = t11059 * t42387;
    let t42397 = t820 * t10970;
    let t42403 = t3070 * t10422 * t10418;
    let t42412 = t3070 * t10422 * t10397;
    (t42380, t42386, t42387, t42388, t42397, t42403, t42412)
}
