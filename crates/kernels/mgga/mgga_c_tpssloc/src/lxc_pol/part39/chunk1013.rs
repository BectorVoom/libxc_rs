//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1013/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1013<F: Float>(t12418: F, t246: F, t3734: F, t550: F, t3777: F, t3802: F, t225: F, t3755: F, t3700: F, t570: F, t1390: F, t3914: F) -> (F, F, F, F, F, F) {
    let t12419 = t12418 * t246;
    let t12420 = t550 * t3734;
    let t12429 = t3777 * t3802;
    let t12444 = t3755 * t225;
    let t12461 = F::new(1.0) / t3700 / t570;
    let t12466 = t3914 * t1390;
    (t12419, t12420, t12429, t12444, t12461, t12466)
}
