//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 539/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk539<F: Float>(t1557: F, t2787: F, t912: F, t2792: F, t1547: F, t2798: F, t896: F, t2766: F, t2802: F, t4335: F, t4340: F, t4345: F, t4349: F, t894: F, t2815: F, t901: F) -> (F, F, F, F, F, F) {
    let t4358 = 1.0 * t2787 * t1557;
    let t4359 = t1557 * t912;
    let t4361 = 2.0 * t2792 * t4359;
    let t4362 = t2798 * t1547;
    let t4363 = t4362 * t896;
    let t4370 = t2802 + t2766 / 9.0 + t4335 / 9.0 - 2.0 / 9.0 * t4340 + 2.0 / 3.0 * t4345 - t4349 / 3.0;
    let t4371 = t894 * t4370;
    let t4378 = t2815 * t1547;
    let t4379 = t4378 * t896;
    let t4381 = t901 * t4370;
    (t4358, t4361, t4363, t4371, t4379, t4381)
}
