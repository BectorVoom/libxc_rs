//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 760/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk760<F: Float>(t2766: F, t2824: F, t2912: F, t2919: F, t4335: F, t4340: F, t4345: F, t4349: F, t4363: F, t4371: F, t4379: F, t4381: F, t4384: F, t4387: F, t4390: F, t4393: F) -> (F,) {
    let t4471 = -0.1294625e1 * t4363 + 0.258925e1 * t4371 + t2912 + 0.10064166666666666667e0 * t2766 + 0.10064166666666666667e0 * t4335 - 0.20128333333333333333e0 * t4340 + 0.60385e0 * t4345 - 0.301925e0 * t4349 + 0.82524375e-1 * t4379 + 0.16504875e0 * t4381 + t2919 + 0.5519e-1 * t2824 + 0.5519e-1 * t4384 - 0.27595e-1 * t4387 + 0.16557e0 * t4390 - 0.82785e-1 * t4393;
    (t4471,)
}
