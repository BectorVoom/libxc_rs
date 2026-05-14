//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 712/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk712<F: Float>(t2663: F, t4211: F, t2535: F, t4199: F, t1471: F, t32: F, t118: F, t1474: F, t2375: F, t1512: F, t9671: F, t1509: F, t2632: F, t1500: F, t2693: F, t2642: F, t4166: F) -> (F, F, F, F, F, F, F, F) {
    let t13109 = t4211 * t2663;
    let t13113 = t4199 * t2535;
    let t13115 = t32 * t1471;
    let t13123 = t1474 * t118;
    let t13124 = t13123 * t2375;
    let t13182 = t9671 * t1512;
    let t13228 = t1509 * t2632;
    let t13234 = t1500 * t2693;
    let t13251 = t4166 * t2642;
    (t13109, t13113, t13115, t13124, t13182, t13228, t13234, t13251)
}
