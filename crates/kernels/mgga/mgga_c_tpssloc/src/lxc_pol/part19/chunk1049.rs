//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1049/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1049<F: Float>(t25: F, t28: F, t11985: F, t526: F, t3665: F, t2249: F, t12061: F, t12064: F, t3664: F, t39109: F, t514: F, t9257: F, t11998: F, t528: F, t3673: F, t3231: F, t11122: F, t12072: F, t12075: F, t3672: F, t517: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t26 = t25 <= zeta_threshold;
    let t29 = t28 <= zeta_threshold;
    let t39419 = 1.0 / t526 / t11985;
    let t39420 = t3665 * t3665;
    let t39426 = t2249 * t2249;
    let t39434 = piecewise3(t26, 0.0, 40.0 / 81.0 * t39419 * t39420 - 16.0 / 9.0 * t12061 * t3665 * t2249 + 4.0 / 3.0 * t3664 * t39426 + 16.0 / 9.0 * t12064 * t9257 + 4.0 / 3.0 * t514 * t39109);
    let t39436 = 1.0 / t528 / t11998;
    let t39437 = t3673 * t3673;
    let t39443 = t3231 * t3231;
    let t39448 = -t39109;
    let t39452 = piecewise3(t29, 0.0, 40.0 / 81.0 * t39436 * t39437 - 16.0 / 9.0 * t12072 * t3673 * t3231 + 4.0 / 3.0 * t3672 * t39443 + 16.0 / 9.0 * t12075 * t11122 + 4.0 / 3.0 * t517 * t39448);
    (t39420, t39426, t39434, t39437, t39443, t39448, t39452)
}
