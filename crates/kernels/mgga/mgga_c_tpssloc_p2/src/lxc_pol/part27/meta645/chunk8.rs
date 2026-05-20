//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2215/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2215<F: Float>(t23418: F, t4669: F, t13765: F, t23419: F, t14033: F, t14069: F, t14488: F, t23457: F, t23495: F, t25585: F, t25589: F, t25609: F, t3073: F, t360: F, t4575: F, t6723: F, t6735: F, t6742: F, t6744: F, t68: F, t7574: F, t7578: F, t83041: F, t83046: F, t83220: F) -> F {
    let t88513 = t4669 * t23418;
    let t88517 = t23419 * t13765 / F::new(1728.0);
    let t88533 = -F::cast_from(0.20186378047070195428e-3_f64) * t25589 * t6735 + F::cast_from(0.16149102437656156342e-2_f64) * t23457 * t7578 + F::cast_from(0.16149102437656156342e-2_f64) * t6723 * t25609 + F::cast_from(0.16149102437656156342e-2_f64) * t25585 * t6735 + t88513 * t3073 / F::new(1152.0) + t88517 - t83220 * t4575 / F::new(216.0) + F::cast_from(0.10093189023535097714e-3_f64) * t6742 * t6744 * t14488 * t68 * t360 + t23419 * t14069 / F::new(1152.0) + t23419 * t14033 / F::new(2304.0) + t83041 / F::new(1728.0) - t83046 / F::new(216.0) - F::cast_from(0.10093189023535097714e-3_f64) * t7574 * t23495;
    t88533
}
