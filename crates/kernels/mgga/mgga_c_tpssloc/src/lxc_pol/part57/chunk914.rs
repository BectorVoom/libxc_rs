//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 914/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk914<F: Float>(t27188: F, t7461: F, t28835: F, t8607: F, t128920: F, t1873: F, t7467: F, t128402: F, t33234: F, t28017: F, t7042: F, t128521: F, t2039: F, t126035: F, t126036: F, t126116: F, t126118: F, t126120: F, t28951: F, t6517: F, t8446: F, t96686: F) -> (F, F, F) {
    let t128924 = 4.0 * t27188 * t7461;
    let t128926 = 3.0 * t8607 * t28835;
    let t128928 = 4.0 * t128920 * t1873;
    let t128930 = 4.0 * t27188 * t7467;
    let t128932 = 2.0 * t128402 * t1873;
    let t128934 = 4.0 * t33234 * t7467;
    let t128936 = 2.0 * t7042 * t28017;
    let t128942 = 2.0 * t128521 * t2039;
    let t128943 = 2.0 * t2039 * t96686 + 2.0 * t28951 * t6517 + t126035 + t126036 + t126116 + t126118 + t126120 + t128928 + t128930 + t128932 + t128934 + t128936 + t128942 + t8446;
    (t128924, t128926, t128943)
}
