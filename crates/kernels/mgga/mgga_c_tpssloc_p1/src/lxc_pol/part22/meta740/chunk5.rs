//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2442/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2442<F: Float>(t300: F, t69050: F, t69180: F, t69218: F, t69249: F, t69286: F, t69326: F, t69368: F, t69449: F, t14459: F, t17947: F, t959: F) -> (F, F) {
    let t69453 = t300 * (t69050 + t69180 + t69218 + t69249 + t69286 + t69326 + t69368 + t69449);
    let t69456 = F::cast_from(0.31168546390226634765e3_f64) * t959 * t17947 * t14459;
    (t69453, t69456)
}
