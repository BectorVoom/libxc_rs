//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 756/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk756<F: Float>(t1547: F, t2798: F, t896: F, t2766: F, t2802: F, t4335: F, t4340: F, t4345: F, t4349: F, t894: F, t2815: F, t901: F, t1553: F, t699: F, t2826: F, t4338: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4362 = t2798 * t1547;
    let t4363 = t4362 * t896;
    let t4370 = t2802 + t2766 / 9.0 + t4335 / 9.0 - 2.0 / 9.0 * t4340 + 2.0 / 3.0 * t4345 - t4349 / 3.0;
    let t4371 = t894 * t4370;
    let t4378 = t2815 * t1547;
    let t4379 = t4378 * t896;
    let t4381 = t901 * t4370;
    let t4384 = t699 * t1553;
    let t4386 = t2826 * t4338;
    (t4362, t4363, t4370, t4371, t4378, t4379, t4381, t4384, t4386)
}
