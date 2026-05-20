//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1498/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1498<F: Float>(t17161: F, t2826: F, t136: F, t10304: F, t17152: F, t17167: F, t908: F, t17171: F, t17183: F, t17178: F, t10556: F, t10577: F, t13598: F, t13600: F, t13601: F, t13603: F, t17149: F, t17154: F, t17159: F, t17163: F, t17165: F, t17169: F, t17173: F, t17175: F, t17180: F, t17185: F, t17189: F) -> (F, F, F, F, F, F, F) {
    let t17240 = t2826 * t17161;
    let t17241 = t136 * t17240;
    let t17243 = t10304 * t17152;
    let t17244 = t136 * t17243;
    let t17246 = t908 * t17167;
    let t17247 = t136 * t17246;
    let t17249 = t908 * t17171;
    let t17250 = t136 * t17249;
    let t17252 = t908 * t17183;
    let t17253 = t136 * t17252;
    let t17255 = t2826 * t17178;
    let t17256 = t136 * t17255;
    let t17271 = -t10577 - F::new(4.0) / F::new(27.0) * t10556 - F::new(8.0) / F::new(27.0) * t13598 + t13600 - t13601 + t13603 + F::new(2.0) / F::new(27.0) * t17149 - F::new(10.0) / F::new(27.0) * t17154 + F::new(4.0) / F::new(3.0) * t17159 - F::new(4.0) / F::new(9.0) * t17163 - F::new(2.0) / F::new(9.0) * t17165 - F::new(2.0) * t17169 + F::new(4.0) / F::new(3.0) * t17173 + t17175 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t17180 + F::new(2.0) / F::new(3.0) * t17185 - t17189 / F::new(3.0);
    (t17241, t17244, t17247, t17250, t17253, t17256, t17271)
}
