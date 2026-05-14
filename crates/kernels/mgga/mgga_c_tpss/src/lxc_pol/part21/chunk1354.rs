//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1354/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1354<F: Float>(t13235: F, t6113: F, t19441: F, t3499: F, t1600: F, t18403: F, t626: F, t7798: F, t13136: F, t13225: F, t1753: F, t5514: F, t65525: F, t65527: F, t65530: F, t65532: F, t65535: F, t65538: F, t65540: F, t65543: F, t65548: F, t65897: F, t65902: F, t65904: F, t65906: F) -> (F,) {
    let t65908 = 2.0 * t13235 * t6113;
    let t65910 = 4.0 * t3499 * t19441;
    let t65915 = 2.0 * t626 * t1600 * t18403;
    let t65917 = 2.0 * t7798 * t6113;
    let t65918 = -2.0 * t13136 * t1753 - 4.0 * t13225 * t5514 - t65525 - t65527 + t65530 + t65532 - t65535 + t65538 + t65540 - t65543 + t65548 + t65897 - t65902 - t65904 - t65906 - t65908 - t65910 - t65915 - t65917;
    (t65918,)
}
