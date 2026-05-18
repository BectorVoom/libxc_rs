//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1048/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1048<F: Float>(t13598: F, t14324: F, t17149: F, t17165: F, t17175: F, t17189: F, t17280: F, t17286: F, t17288: F, t17290: F, t17293: F, t13642: F, t14321: F, t17154: F, t17159: F, t17163: F, t17169: F, t17211: F, t17213: F, t17216: F, t17219: F, t17221: F, t17224: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t17272: F, t17274: F, t17325: F) -> F {
    let t17347 = -F::new(0.45908888888888888888e0) * t13598 + t14324 + F::new(0.20839e0) * t17280 + F::new(0.11477222222222222222e0) * t17149 - F::new(0.34431666666666666667e0) * t17165 + F::new(0.17215833333333333333e0) * t17175 - F::new(0.516475e0) * t17189 + F::new(0.23154444444444444445e-1) * t17286 - F::new(0.13892666666666666667e0) * t17288 + F::new(0.69463333333333333333e-1) * t17290 - F::new(0.104195e0) * t17293;
    let t17349 = F::new(0.264729375e1) * t17211 - F::new(0.3529725e1) * t17213 - F::new(0.17648625e1) * t17216 - F::new(0.157790625e0) * t17219 + F::new(0.6311625e0) * t17221 + F::new(0.31558125e0) * t17224 - F::new(0.57386111111111111112e0) * t17154 + F::new(0.20659e1) * t17159 - F::new(0.68863333333333333334e0) * t17163 - F::new(0.309885e1) * t17169 + t17325 - F::new(0.69463333333333333334e-1) * t17241 - F::new(0.46308888888888888889e-1) * t17244 - F::new(0.62517e0) * t17247 + F::new(0.41678e0) * t17250 + F::new(0.20839e0) * t17253 - F::new(0.34731666666666666667e-1) * t17256 + F::new(0.3529725e1) * t17272 + F::new(0.6311625e0) * t17274 - F::new(0.23154444444444444445e0) * t13642 + t14321 + t17347;
    t17349
}
