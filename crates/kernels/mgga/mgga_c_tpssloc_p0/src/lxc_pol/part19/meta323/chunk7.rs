//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1150/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1150<F: Float>(t28: F, t11122: F, t12000: F, t12004: F, t1302: F, t3231: F, t3673: F, t3711: F, t39437: F, t39443: F, t39448: F, t39877: F, t39874: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t39890 = piecewise3::<F>(t29, F::new(0.0), -F::new(56.0) / F::new(81.0) * t39877 * t39437 + F::new(16.0) / F::new(9.0) * t12000 * t3673 * t3231 - F::new(2.0) / F::new(3.0) * t3711 * t39443 - F::new(8.0) / F::new(9.0) * t12004 * t11122 + F::new(2.0) / F::new(3.0) * t1302 * t39448);
    let t39892 = t39874 / F::new(2.0) + t39890 / F::new(2.0);
    t39892
}
