//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1050/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1050<F: Float>(t157: F, t39434: F, t39452: F, t182: F, t2405: F, t2419: F, t690: F, t703: F) -> (F, F, F) {
    let t39454 = (t39434 + t39452) * t157;
    let t39456 = 0.19751673498613801407e-1 * t39454 * t182;
    let t39463 = 0.4274e0 * t690 * t2419 * t2405 * t703;
    (t39454, t39456, t39463)
}
