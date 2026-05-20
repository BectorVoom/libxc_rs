//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2046/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2046<F: Float>(t25608: F, t381: F, t13797: F, t1926: F, t221: F, t10216: F, t387: F, t10277: F, t1625: F, t225: F, t344: F, t25796: F, t4547: F) -> (F, F, F, F, F, F) {
    let t88004 = t25608 * t381;
    let t88022 = t1926 * t221 * t13797;
    let t88023 = t387 * t10216;
    let t88035 = t387 * t10277;
    let t88050 = t344 * t1625 * t225;
    let t88058 = t4547 * t25796;
    (t88004, t88022, t88023, t88035, t88050, t88058)
}
