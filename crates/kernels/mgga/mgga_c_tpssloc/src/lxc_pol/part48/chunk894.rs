//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 894/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk894<F: Float>(t32415: F, t576: F, t1404: F, t8843: F, t1395: F, t8852: F, t2105: F, t7415: F, t2174: F, t7222: F, t2169: F, t7240: F, t2319: F, t8828: F, t63: F, t8308: F) -> (F, F, F, F, F, F, F, F) {
    let t117412 = t576 * t32415;
    let t117416 = t8843 * t1404;
    let t117418 = t1395 * t8852;
    let t117420 = t7415 * t2105;
    let t117422 = t7222 * t2174;
    let t117430 = t2169 * t7240;
    let t117445 = t8828 * t2319;
    let t117447 = t8308 * t63;
    (t117412, t117416, t117418, t117420, t117422, t117430, t117445, t117447)
}
