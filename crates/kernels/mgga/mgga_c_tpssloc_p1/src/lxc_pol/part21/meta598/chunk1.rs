//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2350/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2350<F: Float>(t14: F, t2230: F, t22811: F, t19: F, t2233: F, t2239: F, t601: F, t9238: F, t85: F, t24: F, t10276: F, t73: F) -> (F, F, F, F, F, F) {
    let t39037 = t14 * t2230;
    let t39041 = F::cast_from(1.0_f64) / t22811;
    let t39043 = F::cast_from(0.683424e4_f64) * t19 * t39041;
    let t39049 = t2233 * t2239;
    let t39054 = t601 * t9238;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t39096 = F::cast_from(1.0_f64) / t73 / t10276;
    (t39037, t39043, t39049, t39054, t39063, t39096)
}
