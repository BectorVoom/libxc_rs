//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 779/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk779<F: Float>(t1653: F, t7286: F, t7285: F, t1716: F, t2123: F, t1751: F, t225: F, t497: F, t462: F, t1760: F, t7301: F, t7300: F, t1720: F, t2144: F, t131: F, t7998: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8002 = t7286 * t1653;
    let t8003 = t7285 * t8002;
    let t8006 = t1716 * t2123;
    let t8009 = t1751 * t225;
    let t8010 = t8009 * t497;
    let t8011 = t462 * t8010;
    let t8014 = t7301 * t1760;
    let t8015 = t7300 * t8014;
    let t8018 = t1720 * t2144;
    let t8020 = t7998 * t131;
    (t8002, t8003, t8006, t8010, t8011, t8014, t8015, t8018, t8020)
}
