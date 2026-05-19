//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 115/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk115<F: Float>(t221: F, t341: F, t225: F, t349: F, t68: F, t336: F, t293: F, t328: F, t330: F) -> (F, F, F, F, F, F) {
    let t350 = t221 * t341;
    let t353 = t349 * t225;
    let t354 = t353 * t68;
    let t357 = F::new(1.0) / t336;
    let t358 = t68 * t357;
    let t360 = F::exp(-(-t293 + t328 + t330) * t225 * t358);
    (t350, t353, t354, t357, t358, t360)
}
