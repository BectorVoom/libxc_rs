//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1329/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1329<F: Float>(t5314: F, t5531: F, t626: F, t19626: F, t65533: F, t19610: F, t13965: F, t18547: F, t24790: F, t1668: F, t20119: F, t13546: F, t547: F, t5772: F, t1338: F, t1688: F) -> (F, F, F, F, F, F, F) {
    let t71002 = 2.0 * t626 * t5314 * t5531;
    let t71010 = 6.0 * t65533 * t19626;
    let t71012 = 6.0 * t65533 * t19610;
    let t71017 = 6.0 * t18547 * t24790 * t13965;
    let t71032 = 12.0 * t1668 * t20119;
    let t71037 = 6.0 * t547 * t5772 * t13546;
    let t71038 = t1338 * t1688;
    (t71002, t71010, t71012, t71017, t71032, t71037, t71038)
}
