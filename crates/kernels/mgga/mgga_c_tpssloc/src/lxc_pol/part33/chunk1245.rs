//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1245/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1245<F: Float>(t16311: F, t3788: F, t6414: F, t6936: F, t1339: F, t20554: F, t20563: F, t221: F, t26284: F, t20442: F, t22833: F, t2002: F, t20595: F, t559: F, t80900: F, t80957: F, t80971: F, t91394: F, t91398: F, t91400: F, t97394: F, t97400: F, t97402: F, t97404: F, t97427: F, t97431: F, t97439: F, t97444: F, t97463: F) -> (F,) {
    let t107183 = t6936 * t3788 * t16311 * t6414;
    let t107186 = t6936 * t1339 * t20554;
    let t107189 = t26284 * t221 * t20563;
    let t107198 = t22833 * t20442;
    let t107205 = t20595 * t2002 * t559;
    let t107208 = 0.12111826828242117256e-2 * t107183 - t80900 - 0.20186378047070195427e-3 * t107186 + 3.0 / 16.0 * t107189 + 7.0 / 48.0 * t97394 - 0.84782787797694820794e-2 * t97400 - 119.0 / 2304.0 * t91394 - 7.0 / 16.0 * t97402 - 0.17804385437515912366e0 * t97404 - 0.42391393898847410397e-2 * t97427 + 0.60559134141210586281e-3 * t97431 - t107198 / 512.0 + 0.25434836339308446238e-1 * t97439 - 35.0 / 72.0 * t91398 - 0.2034786907144675699e0 * t91400 + 0.42391393898847410397e-2 * t97444 + t107205 / 1536.0 - t80957 + t80971 + 0.42391393898847410397e-2 * t97463;
    (t107208,)
}
