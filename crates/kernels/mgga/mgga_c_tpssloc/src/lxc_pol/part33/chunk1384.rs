//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1384/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1384<F: Float>(t20565: F, t6952: F, t20556: F, t6945: F, t1827: F, t97246: F, t26233: F, t6417: F, t20492: F, t80903: F, t107063: F, t107065: F, t107067: F, t107070: F, t107074: F, t91149: F, t91167: F, t97219: F, t97238: F, t97240: F, t97253: F, t97261: F, t97263: F, t97283: F) -> F {
    let t107077 = t6952 * t20565;
    let t107084 = t6945 * t20556;
    let t107086 = t97246 * t1827;
    let t107088 = t26233 * t6417;
    let t107090 = t80903 * t20492;
    let t107092 = t107063 / F::new(128.0) + t107065 / F::new(256.0) + t107067 / F::new(128.0) - F::new(7.0) / F::new(96.0) * t97219 + t107070 / F::new(128.0) - F::new(0.50869672678616892476e-1) * t97238 + F::new(7.0) / F::new(384.0) * t97240 - t107074 / F::new(512.0) + F::new(7.0) / F::new(768.0) * t97253 + F::new(5.0) / F::new(128.0) * t107077 + F::new(7.0) / F::new(192.0) * t97261 + F::new(7.0) / F::new(96.0) * t97263 - F::new(119.0) / F::new(576.0) * t91149 - F::new(35.0) / F::new(192.0) * t97283 - F::new(0.33913115119077928317e-1) * t91167 - t107084 / F::new(1536.0) - t107086 / F::new(512.0) - t107088 / F::new(512.0) - t107090 / F::new(256.0);
    t107092
}
