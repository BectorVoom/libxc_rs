//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2744/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2744<F: Float>(t16583: F, t16589: F, t39249: F, t39256: F, t40626: F, t46341: F, t57877: F, t57879: F, t57880: F, t57885: F, t57886: F, t57888: F, t57889: F) -> F {
    let t57890 = F::cast_from(24.0_f64) * t16583 * t46341 + F::cast_from(12.0_f64) * t16589 * t46341 - t39249 - t39256 + t40626 + t57877 + t57879 - t57880 - t57885 - t57886 + t57888 - t57889;
    t57890
}
