//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1113/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1113<F: Float>(t1336: F, t22759: F, t835: F, t3795: F, t22760: F, t3777: F, t22756: F, t3853: F, t12379: F, t6945: F, t22765: F, t80943: F, t80947: F, t80950: F, t80957: F, t80959: F, t80963: F, t80971: F, t80974: F, t80978: F, t80982: F, t80985: F, t80987: F, t80989: F, t80992: F, t80994: F) -> (F,) {
    let t80997 = t1336 * t22759 * t835;
    let t80998 = t80997 * t3795;
    let t81000 = t3777 * t22760;
    let t81001 = t81000 * t3795;
    let t81003 = t22756 * t3853;
    let t81005 = t6945 * t12379;
    let t81007 = t22765 * t3853;
    let t81009 = -0.84782787797694820794e-2 * t80943 + 0.36335480484726351768e-2 * t80947 - 0.12111826828242117256e-2 * t80950 - t80957 - 0.50869672678616892476e-1 * t80959 - 0.25434836339308446237e-1 * t80963 + t80971 - 0.72670960969452703536e-2 * t80974 + 0.36335480484726351768e-2 * t80978 + 0.36335480484726351768e-2 * t80982 + 0.12111826828242117256e-2 * t80985 - t80987 / 1536.0 + 7.0 / 768.0 * t80989 + 7.0 / 384.0 * t80992 - t80994 / 512.0 - 7.0 / 384.0 * t80998 + t81001 / 256.0 - t81003 / 512.0 - t81005 / 1536.0 + 7.0 / 768.0 * t81007;
    (t81009,)
}
