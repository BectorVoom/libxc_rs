//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 251/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk251<F: Float>(t103: F, t662: F, t100: F, t657: F, t660: F, t92: F, t96: F) -> (F, F) {
    let t663 = t103 * t662;
    let t666 = F::new(5.0) / F::new(3.0) * t100 * t663 - F::new(5.0) / F::new(3.0) * t657 * t96 + F::new(5.0) / F::new(3.0) * t92 * t660;
    (t663, t666)
}
