//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1223/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1223<F: Float>(t21298: F, t935: F, t1702: F, t4783: F, t18000: F, t1378: F, t1395: F, t226: F, t18007: F, t6130: F, t5572: F, t4799: F, t4715: F, t2162: F, t18021: F, t5577: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t21299 = t21298 * t935;
    let t21307 = t1702 * t4783;
    let t21308 = t18000 * t21307;
    let t21312 = t1395 * t1378 * t226;
    let t21313 = t18007 * t21312;
    let t21316 = t6130 * t1395;
    let t21317 = t5572 * t21316;
    let t21320 = t1702 * t4799;
    let t21321 = t5572 * t21320;
    let t21324 = t1702 * t4715;
    let t21325 = t21324 * t2162;
    let t21326 = t18021 * t21325;
    let t21330 = t6130 * t1378 * t226;
    let t21331 = t5577 * t21330;
    (t21299, t21307, t21308, t21312, t21313, t21316, t21317, t21320, t21321, t21324, t21326, t21331)
}
