//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 891/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk891<F: Float>(t1125: F, t9676: F, t2845: F, t400: F, t2192: F, t359: F, t461: F, t460: F, t3097: F, t774: F, t3054: F, t3073: F, t219: F, t3111: F, t1137: F, t73: F) -> (F, F, F, F, F, F, F) {
    let t9677 = t1125 * t9676;
    let t9684 = 1.0 / t400 / t2845;
    let t9699 = t359 * t2192 * t461;
    let t9701 = t460 * t9699 / 10368.0;
    let t9702 = t774 * t3097;
    let t9721 = t3054 * t3073;
    let t9730 = t3111 * t219;
    let t9737 = t1137 * t1137;
    let t9738 = 1.0 / t9737;
    let t9739 = t73 * t9738;
    (t9677, t9684, t9701, t9702, t9721, t9730, t9739)
}
