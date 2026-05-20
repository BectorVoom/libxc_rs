//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1149/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1149<F: Float>(t25: F, t11987: F, t11991: F, t1298: F, t2249: F, t3665: F, t3704: F, t39109: F, t39420: F, t39426: F, t39861: F, t9257: F, t11998: F, t28: F, t517: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t39874 = piecewise3::<F>(t26, F::new(0.0), -F::new(56.0) / F::new(81.0) * t39861 * t39420 + F::new(16.0) / F::new(9.0) * t11987 * t3665 * t2249 - F::new(2.0) / F::new(3.0) * t3704 * t39426 - F::new(8.0) / F::new(9.0) * t11991 * t9257 + F::new(2.0) / F::new(3.0) * t1298 * t39109);
    let t39877 = F::new(1.0) / t517 / t11998 / t28;
    (t39874, t39877)
}
