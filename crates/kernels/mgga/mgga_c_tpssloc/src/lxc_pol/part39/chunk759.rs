//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 759/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk759<F: Float>(t2766: F, t2824: F, t2868: F, t2875: F, t4335: F, t4340: F, t4345: F, t4349: F, t4363: F, t4371: F, t4379: F, t4381: F, t4384: F, t4387: F, t4390: F, t4393: F) -> (F,) {
    let t4433 = -0.17648625e1 * t4363 + 0.3529725e1 * t4371 + t2868 + 0.17215833333333333333e0 * t2766 + 0.17215833333333333333e0 * t4335 - 0.34431666666666666667e0 * t4340 + 0.103295e1 * t4345 - 0.516475e0 * t4349 + 0.31558125e0 * t4379 + 0.6311625e0 * t4381 + t2875 + 0.69463333333333333333e-1 * t2824 + 0.69463333333333333333e-1 * t4384 - 0.34731666666666666667e-1 * t4387 + 0.20839e0 * t4390 - 0.104195e0 * t4393;
    (t4433,)
}
