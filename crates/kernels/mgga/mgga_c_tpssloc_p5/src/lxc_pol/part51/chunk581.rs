//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 581/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk581<F: Float>(t4370: F, t894: F, t1547: F, t2815: F, t896: F, t901: F, t1553: F, t699: F, t2826: F, t4338: F, t136: F, t4343: F, t908: F) -> (F, F, F, F, F, F) {
    let t4371 = t894 * t4370;
    let t4378 = t2815 * t1547;
    let t4379 = t4378 * t896;
    let t4381 = t901 * t4370;
    let t4384 = t699 * t1553;
    let t4386 = t2826 * t4338;
    let t4387 = t136 * t4386;
    let t4389 = t908 * t4343;
    (t4371, t4379, t4381, t4384, t4387, t4389)
}
