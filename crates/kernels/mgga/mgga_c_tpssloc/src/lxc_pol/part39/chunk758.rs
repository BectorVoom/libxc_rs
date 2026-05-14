//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 758/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk758<F: Float>(t4395: F, t913: F, t893: F, t1556: F, t2844: F, t912: F, t2842: F, t2766: F, t2848: F, t4335: F, t4340: F, t4345: F, t4349: F, t1561: F, t923: F, t1569: F, t931: F) -> (F, F, F, F, F, F, F, F) {
    let t4396 = t4395 * t913;
    let t4398 = 1.0 * t893 * t4396;
    let t4399 = t1556 * t2844;
    let t4400 = t4399 * t912;
    let t4402 = 0.16081979498692535067e2 * t2842 * t4400;
    let t4408 = t2848 + 0.57077777777777777777e-2 * t2766 + 0.57077777777777777777e-2 * t4335 - 0.11415555555555555555e-1 * t4340 + 0.34246666666666666666e-1 * t4345 - 0.17123333333333333333e-1 * t4349;
    let t4411 = t1561 * t923;
    let t4416 = t1569 * t931;
    (t4396, t4398, t4399, t4400, t4402, t4408, t4411, t4416)
}
