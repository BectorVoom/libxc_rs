//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1392/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1392<F: Float>(t107183: F, t107186: F, t107189: F, t107198: F, t107205: F, t80900: F, t80957: F, t80971: F, t91394: F, t91398: F, t91400: F, t97394: F, t97400: F, t97402: F, t97404: F, t97427: F, t97431: F, t97439: F, t97444: F, t97463: F) -> F {
    let t107208 = F::new(0.12111826828242117256e-2) * t107183 - t80900 - F::new(0.20186378047070195427e-3) * t107186 + F::new(3.0) / F::new(16.0) * t107189 + F::new(7.0) / F::new(48.0) * t97394 - F::new(0.84782787797694820794e-2) * t97400 - F::new(119.0) / F::new(2304.0) * t91394 - F::new(7.0) / F::new(16.0) * t97402 - F::new(0.17804385437515912366e0) * t97404 - F::new(0.42391393898847410397e-2) * t97427 + F::new(0.60559134141210586281e-3) * t97431 - t107198 / F::new(512.0) + F::new(0.25434836339308446238e-1) * t97439 - F::new(35.0) / F::new(72.0) * t91398 - F::new(0.2034786907144675699e0) * t91400 + F::new(0.42391393898847410397e-2) * t97444 + t107205 / F::new(1536.0) - t80957 + t80971 + F::new(0.42391393898847410397e-2) * t97463;
    t107208
}
