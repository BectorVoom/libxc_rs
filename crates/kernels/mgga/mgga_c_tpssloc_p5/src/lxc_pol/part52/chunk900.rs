//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 900/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk900<F: Float>(t1873: F, t7266: F, t8441: F, t8444: F, t8446: F, t8667: F, t191: F, t2167: F, t192: F) -> (F, F, F) {
    let t8684 = t7266 * t1873;
    let t8687 = t8667 + F::cast_from(2.0_f64) * t8684 + F::cast_from(2.0_f64) * t8441 + t8444 + t8446;
    let t8689 = t2167 * t191;
    let t8690 = t8689 * t192;
    (t8687, t8689, t8690)
}
