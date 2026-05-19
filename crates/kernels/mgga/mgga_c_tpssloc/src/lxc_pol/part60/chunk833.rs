//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 833/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk833<F: Float>(t2140: F, t6169: F, t1748: F, t27611: F, t27617: F, t27622: F, t27629: F, t27684: F, t27711: F, t29585: F, t29594: F, t29597: F, t29601: F, t467: F, t488: F, t7326: F, t8040: F) -> F {
    let t29606 = t6169 * t2140;
    let t29610 = -t27617 * t1748 / F::new(1152.0) + t27611 / F::new(1152.0) + F::new(11.0) / F::new(108.0) * t29585 * t467 - F::cast_from(0.16149102437656156342e-2_f64) * t27711 * t8040 - F::cast_from(0.20186378047070195428e-3_f64) * t27684 * t8040 + F::cast_from(0.10093189023535097714e-3_f64) * t7326 * t29594 - t29597 * t488 / F::new(144.0) + F::new(19.0) / F::new(864.0) * t29601 * t488 - F::cast_from(0.20186378047070195428e-3_f64) * t27629 * t8040 + t29606 * t488 / F::new(1536.0) - t27622 / F::new(1728.0);
    t29610
}
