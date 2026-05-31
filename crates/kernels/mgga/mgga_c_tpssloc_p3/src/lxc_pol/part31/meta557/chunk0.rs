//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1785/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1785<F: Float>(t82122: F, t82153: F, t82218: F, t81440: F, t1453: F, t81439: F, t26129: F, t81442: F, t22470: F, t4067: F, t25: F, t40772: F) -> (F, F, F, F, F, F, F, F) {
    let t85060 = F::cast_from(0.3244175520728446583e0_f64) * t82122;
    let t85101 = F::cast_from(0.27415567780803773942e-2_f64) * t82153;
    let t85129 = F::cast_from(0.55440370401180965083e0_f64) * t82218;
    let t86583 = F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t81440;
    let t86586 = t81439 * t1453;
    let t86588 = t81442 * t26129;
    let t86590 = t22470 * t4067;
    let t86716 = t40772 * t25;
    (t85060, t85101, t85129, t86583, t86586, t86588, t86590, t86716)
}
