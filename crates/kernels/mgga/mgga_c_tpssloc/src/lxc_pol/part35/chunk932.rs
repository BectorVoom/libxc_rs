//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 932/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk932<F: Float>(t10784: F, t10785: F, t21120: F, t21124: F, t21128: F, t21132: F, t21136: F, t21140: F, t21142: F, t21144: F, t21147: F, t21150: F, t21153: F, t21156: F, t21126: F, t908: F) -> (F, F) {
    let t21158 = 0.20839e0 * t21120 - 0.103295e1 * t21124 + 0.309885e1 * t21128 - 0.46308888888888888889e-1 * t21132 - 0.104195e0 * t21136 - 0.62517e0 * t21140 - 0.52945875e1 * t21142 + 0.94674375e0 * t21144 - t10784 - t10785 - 0.57386111111111111112e0 * t21147 + 0.20659e1 * t21150 - 0.309885e1 * t21153 - 0.516475e0 * t21156;
    let t21160 = t908 * t21126;
    (t21158, t21160)
}
