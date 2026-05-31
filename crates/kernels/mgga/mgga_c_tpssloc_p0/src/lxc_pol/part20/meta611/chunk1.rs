//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2199/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2199<F: Float>(t3623: F, t44927: F, t11880: F, t44690: F, t11913: F, t11931: F, t225: F, t11604: F, t496: F, t68: F, t11601: F, t11599: F) -> (F, F, F, F, F, F, F) {
    let t45323 = t44927 * t3623;
    let t45326 = t44690 * t11880;
    let t45329 = t44690 * t11913;
    let t45345 = t11931 * t225;
    let t45349 = F::cast_from(1.0_f64) / t11604 / t496;
    let t45350 = t68 * t45349;
    let t45355 = t11601 * t225;
    let t45375 = t11599 * t225;
    (t45323, t45326, t45329, t45345, t45350, t45355, t45375)
}
