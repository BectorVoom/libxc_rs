//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2387/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2387<F: Float>(t47761: F, t47765: F, t47769: F, t48112: F, t48114: F, t48116: F, t48119: F, t48122: F, t48125: F, t48128: F, t48131: F, t10588: F, t4362: F) -> (F, F) {
    let t49004 = F::cast_from(0.17938e1_f64) * t47761 + F::cast_from(0.17938e1_f64) * t47765 + F::cast_from(0.59793333333333333334e0_f64) * t47769 + F::cast_from(0.49293999999999999999e0_f64) * t48112 + F::cast_from(0.16431333333333333333e0_f64) * t48114 + F::cast_from(0.73028148148148148149e-1_f64) * t48116 + F::cast_from(0.49294e0_f64) * t48119 + F::cast_from(0.43816888888888888889e0_f64) * t48122 - F::cast_from(0.147882e1_f64) * t48125 - F::cast_from(0.82156666666666666668e-1_f64) * t48128 - F::cast_from(0.10954222222222222222e0_f64) * t48131;
    let t49009 = t4362 * t10588;
    (t49004, t49009)
}
