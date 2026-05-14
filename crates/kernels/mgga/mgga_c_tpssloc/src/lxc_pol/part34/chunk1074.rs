//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1074/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1074<F: Float>(t101226: F, t105755: F, t105759: F, t105763: F, t105766: F, t105780: F, t105797: F, t105814: F, t105818: F, t105822: F, t1877: F, t2057: F, t24191: F, t24344: F, t2522: F, t26563: F, t26744: F, t28252: F, t28256: F, t28456: F, t28459: F, t28462: F, t7114: F, t7545: F, t7845: F, t84766: F, t93000: F) -> (F,) {
    let t108096 = 9.0 * t2522 * t7845 * t28252 - 3.0 / 2.0 * t1877 * t101226 * t7545 - 9.0 * t24191 * t105766 - 9.0 / 2.0 * t24191 * t105759 + 9.0 / 2.0 * t2522 * t7845 * t28256 - 3.0 * t1877 * t84766 * t105822 - 3.0 * t1877 * t26744 * t28459 - 3.0 / 2.0 * t1877 * t26744 * t28462 - 9.0 / 2.0 * t24191 * t105755 + 3.0 * t1877 * t24344 * t105814 - 3.0 / 2.0 * t1877 * t7114 * t105818 + 3.0 * t1877 * t93000 * t28456 - 9.0 * t26563 * t105763 + 3.0 / 2.0 * t2522 * t2057 * t105797 - 3.0 / 2.0 * t1877 * t7114 * t105780;
    (t108096,)
}
