//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 863/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk863<F: Float>(t3315: F, t6020: F, t11277: F, t5988: F, t1222: F, t6170: F, t6158: F, t6165: F, t5416: F, t972: F) -> (F, F, F, F, F, F) {
    let t18258 = t6020 * t3315;
    let t18265 = t5988 * t11277;
    let t18310 = t6170 * t1222;
    let t18312 = t6158 * t1222;
    let t18314 = t6165 * t1222;
    let t18321 = t5416 * t972;
    (t18258, t18265, t18310, t18312, t18314, t18321)
}
