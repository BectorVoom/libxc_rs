//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2282/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2282<F: Float>(t47160: F, t41291: F, t12932: F, t2427: F, t13133: F, t2430: F, t145: F, t185: F, t46191: F, t45872: F, t707: F, t12886: F, t706: F) -> (F, F, F, F, F, F, F) {
    let t47161 = F::new(3.0) * t47160;
    let t47162 = F::new(12.0) * t41291;
    let t47163 = t2427 * t12932;
    let t47164 = F::new(24.0) * t47163;
    let t47165 = t13133 * t2430;
    let t47166 = F::new(24.0) * t47165;
    let t47168 = t145 * t46191 * t185;
    let t47171 = F::new(4.0) * t707 * t185 * t45872;
    let t47172 = t706 * t12886;
    (t47161, t47162, t47164, t47166, t47168, t47171, t47172)
}
