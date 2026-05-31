//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1410/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1410<F: Float>(t107183: F, t107186: F, t107189: F, t107198: F, t107205: F, t80900: F, t80957: F, t80971: F, t91394: F, t91398: F, t91400: F, t97394: F, t97400: F, t97402: F, t97404: F, t97427: F, t97431: F, t97439: F, t97444: F, t97463: F) -> F {
    let t107208 = F::cast_from(0.12111826828242117256e-2_f64) * t107183 - t80900 - F::cast_from(0.20186378047070195427e-3_f64) * t107186 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t107189 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t97394 - F::cast_from(0.84782787797694820794e-2_f64) * t97400 - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t91394 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t97402 - F::cast_from(0.17804385437515912366e0_f64) * t97404 - F::cast_from(0.42391393898847410397e-2_f64) * t97427 + F::cast_from(0.60559134141210586281e-3_f64) * t97431 - t107198 / F::cast_from(512.0_f64) + F::cast_from(0.25434836339308446238e-1_f64) * t97439 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t91398 - F::cast_from(0.2034786907144675699e0_f64) * t91400 + F::cast_from(0.42391393898847410397e-2_f64) * t97444 + t107205 / F::cast_from(1536.0_f64) - t80957 + t80971 + F::cast_from(0.42391393898847410397e-2_f64) * t97463;
    t107208
}
