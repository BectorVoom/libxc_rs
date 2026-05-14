//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1175/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1175<F: Float>(t4478: F, t7310: F, t19620: F, t5706: F, t6277: F, t13965: F, t7029: F, t18547: F, t5710: F, t6243: F, t508: F, t6273: F, t5709: F, t1760: F, t5758: F, t13133: F, t1688: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19621 = t7310 * t4478;
    let t19623 = 6.0 * t19620 * t19621;
    let t19624 = t5706 * t6277;
    let t19626 = t7029 * t13965;
    let t19628 = 3.0 * t18547 * t19626;
    let t19630 = 3.0 * t6243 * t5710;
    let t19631 = t508 * t6273;
    let t19632 = t19631 * t5709;
    let t19634 = 3.0 * t1760 * t19632;
    let t19635 = t6243 * t5758;
    let t19649 = 2.0 * t13133 * t1688;
    (t19621, t19623, t19624, t19626, t19628, t19630, t19631, t19632, t19634, t19635, t19649)
}
