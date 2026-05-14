//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1237/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1237<F: Float>(t26135: F, t7423: F, t24969: F, t7467: F, t112: F, t33761: F, t116362: F, t120786: F, t120788: F, t120789: F, t1458: F, t31284: F, t31937: F, t33195: F, t4072: F, t671: F, t8508: F) -> (F,) {
    let t123272 = t7423 * t26135;
    let t123274 = t24969 * t7467;
    let t123277 = t33761 * t112;
    let t123280 = t31284 + 0.135e2 * t31937 * t4072 + t8508 + 0.135e2 * t116362 * t1458 + t120786 + 0.135e2 * t123272 + 0.135e2 * t123274 + t120788 + 27.0 * t120789 + 0.135e2 * t123277 * t671 + t33195;
    (t123280,)
}
