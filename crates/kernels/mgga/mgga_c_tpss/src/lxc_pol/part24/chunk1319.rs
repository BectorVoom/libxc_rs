//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1319/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1319<F: Float>(t69834: F, t69885: F, t70251: F, t70296: F, t15011: F, t5620: F, t15017: F, t14979: F, t5610: F, t14901: F, t18098: F, t15027: F, t18094: F, t14947: F, t14935: F, t14939: F, t14943: F, t14975: F, t15021: F, t15053: F, t15058: F, t15079: F, t18083: F, t18107: F, t4980: F, t4991: F, t5001: F, t5005: F, t5009: F, t61372: F, t61449: F, t64325: F) -> (F, F) {
    let t70298 = t69834 + t69885 + t70251 + t70296;
    let t70311 = t5620 * t15011;
    let t70319 = t5620 * t15017;
    let t70327 = t5610 * t14979;
    let t70333 = t18098 * t14901;
    let t70335 = t18094 * t15027;
    let t70337 = t5620 * t14947;
    let t70339 = -t64325 - 5.0 / 1152.0 * t5620 * t14935 + 5.0 / 3456.0 * t5620 * t14939 + 5.0 / 2592.0 * t5620 * t14943 + t18094 * t15053 / 768.0 + t18083 * t5005 / 216.0 - t70311 / 1728.0 + t5620 * t15021 / 2304.0 - t61372 * t4980 / 144.0 - t18083 * t5009 / 432.0 + t70319 / 3456.0 - t61449 * t14975 / 1152.0 - t18098 * t15058 / 1536.0 - t18107 * t4991 / 288.0 + t70327 / 2304.0 - 5.0 / 1296.0 * t18083 * t5001 + t5610 * t15079 / 1536.0 - t70333 / 2304.0 + t70335 / 1152.0 + 5.0 / 10368.0 * t70337;
    (t70298, t70339)
}
