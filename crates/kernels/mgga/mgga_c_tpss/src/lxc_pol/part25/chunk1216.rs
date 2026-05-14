//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1216/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1216<F: Float>(t14029: F, t33: F, t18246: F, t52613: F, t1006: F, t4706: F, t14256: F, t20011: F, t14076: F, t64975: F, t1497: F, t8096: F, t19818: F, t51780: F, t3724: F, t63006: F, t65437: F, t65440: F, t67532: F, t67533: F, t68872: F, t68875: F, t68878: F, t68880: F, t68883: F, t68885: F) -> (F, F, F, F, F, F, F, F, F) {
    let t70909 = t33 * t14029;
    let t70915 = t18246 * t52613;
    let t70923 = t1006 * t4706;
    let t70929 = t20011 * t14256;
    let t70932 = t64975 * t14076;
    let t70941 = t8096 * t1497;
    let t70942 = t70941 * t19818;
    let t70957 = t18246 * t51780;
    let t70960 = t1497 * t3724;
    let t71158 = -t63006 - t65437 - 44.0 / 9.0 * t65440 - t67532 + t67533 - 4.0 / 3.0 * t68872 - 3.0 / 2.0 * t68875 + t68878 + 2.0 / 3.0 * t68880 + t68883 / 2.0 - t68885 / 4.0;
    (t70909, t70915, t70923, t70929, t70932, t70942, t70957, t70960, t71158)
}
