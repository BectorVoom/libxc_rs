//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1462/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1462<F: Float>(t120952: F, t2039: F, t102344: F, t1873: F, t115241: F, t122617: F, t122685: F, t122718: F, t122719: F, t122720: F, t122721: F, t122723: F, t122724: F, t122725: F, t122726: F, t122727: F, t1458: F, t26103: F, t27170: F, t31532: F, t33151: F, t33153: F, t4072: F, t6517: F, t671: F, t7801: F, t8445: F) -> F {
    let t122730 = t120952 * t2039;
    let t122731 = t102344 * t1873;
    let t122732 = t115241 * t1458 + t122617 * t671 + t122685 * t1458 + t26103 * t7801 + t27170 * t6517 + t31532 * t4072 + t122718 + t122719 + t122720 + t122721 + t122723 + t122724 + t122725 + t122726 + t122727 + t122730 + t122731 + t33151 + t33153 + t8445;
    t122732
}
