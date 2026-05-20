//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2418/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2418<F: Float>(t300: F, t48786: F, t48861: F, t49076: F, t49113: F, t49266: F, t49409: F, t49450: F, t49492: F, t41769: F, t4496: F, t959: F) -> (F, F) {
    let t49496 = t300 * (t48786 + t48861 + t49076 + t49113 + t49266 + t49409 + t49450 + t49492);
    let t49499 = F::cast_from(0.17315859105681463759e2_f64) * t959 * t4496 * t41769;
    (t49496, t49499)
}
