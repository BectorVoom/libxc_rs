//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1143/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1143<F: Float>(t12571: F, t32582: F, t79: F, t7973: F, t117710: F, t117734: F, t117737: F, t119892: F, t119902: F, t119909: F, t119917: F, t119924: F, t119928: F, t119932: F, t119933: F, t119948: F, t31: F, t31013: F, t31860: F, t31864: F, t32579: F, t32583: F, t33106: F, t33111: F, t33118: F, t34221: F, t607: F, t641: F, t645: F, t7254: F, t8308: F, t8513: F, t8663: F, t8855: F) -> (F,) {
    let t125865 = t12571 * t32582;
    let t125889 = t79 * t7973;
    let t125900 = -5.0 / 18.0 * t117710 * t119892 - 5.0 / 18.0 * t31864 * t8308 * t7973 * t31 * t607 - 5.0 / 18.0 * t117710 * t119902 - 5.0 / 36.0 * t125865 * t31013 + 35.0 / 24.0 * t117737 * t119909 - 5.0 / 12.0 * t31860 * t8513 * t33106 * t7254 - 5.0 / 12.0 * t32579 * t119917 - 5.0 / 36.0 * t117734 * t33111 - 5.0 / 36.0 * t32583 * t119924 - 5.0 / 36.0 * t32583 * t119928 + 5.0 / 18.0 * t119932 * t8855 * t119933 - 5.0 / 12.0 * t31860 * t8513 * t34221 * t645 + 5.0 / 36.0 * t8663 * t8513 * t125889 * t641 - 5.0 / 12.0 * t32579 * t119948 + 5.0 / 36.0 * t8663 * t8513 * t33118 * t7254;
    (t125900,)
}
