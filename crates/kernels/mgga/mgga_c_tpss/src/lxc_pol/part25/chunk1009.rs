//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1009/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1009<F: Float>(t11475: F, t14906: F, t3931: F, t14911: F, t3972: F, t11661: F, t242: F, t4826: F, t8528: F, t967: F, t11456: F, t11459: F, t11462: F, t14902: F, t14908: F, t14913: F, t14917: F, t14922: F, t14925: F, t14928: F, t14931: F, t2731: F, t2748: F, t4996: F, t5005: F, t8456: F, t8472: F, t8577: F, t8588: F, t8976: F, t925: F) -> (F,) {
    let t14934 = t11475 * t14906;
    let t14935 = t3931 * t14934;
    let t14938 = t3972 * t14911;
    let t14939 = t3931 * t14938;
    let t14942 = t11661 * t14906;
    let t14943 = t3931 * t14942;
    let t14947 = t242 * t8528 * t4826;
    let t14948 = t967 * t14947;
    let t14953 = t8976 * t4996 / 576.0 - t14902 / 4608.0 - t8456 / 1296.0 - t8472 / 13824.0 + t967 * t14908 / 768.0 - t967 * t14913 / 1152.0 - t11456 - t11459 + t11462 - t2731 * t14917 / 1536.0 + t8577 * t14922 / 3072.0 - t925 * t14925 / 36.0 + t925 * t14928 / 108.0 + 7.0 / 648.0 * t925 * t14931 - 5.0 / 2304.0 * t967 * t14935 + 5.0 / 6912.0 * t967 * t14939 + 5.0 / 5184.0 * t967 * t14943 + 5.0 / 20736.0 * t14948 + t2748 * t5005 / 432.0 + t8588 / 162.0;
    (t14953,)
}
