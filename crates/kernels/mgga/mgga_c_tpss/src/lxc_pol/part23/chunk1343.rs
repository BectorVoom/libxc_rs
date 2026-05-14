//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1343/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1343<F: Float>(t1322: F, t13220: F, t1600: F, t1897: F, t19187: F, t19244: F, t19251: F, t3493: F, t485: F, t626: F, t65115: F, t65125: F, t65128: F, t65129: F, t65131: F, t65134: F, t65138: F, t65141: F, t65143: F, t65436: F, t65461: F, t65472: F, t65474: F, t65480: F, t68152: F) -> (F,) {
    let t68657 = -2.0 * t13220 * t1897 * t626 - t1322 * t19187 - 2.0 * t1600 * t19244 - 2.0 * t19251 * t3493 - 2.0 * t485 * t68152 - t65115 + t65125 + t65128 - t65129 + t65131 + t65134 + t65138 + t65141 - t65143 + t65436 - t65461 + t65472 + t65474 + t65480;
    (t68657,)
}
