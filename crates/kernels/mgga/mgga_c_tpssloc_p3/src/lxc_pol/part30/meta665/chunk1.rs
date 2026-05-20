//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2091/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2091<F: Float>(t91158: F, t22782: F, t5234: F, t1369: F, t7712: F, t80939: F, t22683: F, t26285: F, t6546: F, t26289: F, t6604: F, t80887: F) -> (F, F, F, F, F, F) {
    let t91159 = F::cast_from(0.13457585364713463618e-3_f64) * t91158;
    let t91160 = t5234 * t22782;
    let t91161 = t91160 * t1369;
    let t91162 = F::new(7.0) / F::new(288.0) * t91161;
    let t91167 = t80939 * t7712;
    let t91170 = t6546 * t22683 * t26285;
    let t91171 = F::new(7.0) / F::new(24.0) * t91170;
    let t91179 = t80887 * t6604 * t26289;
    (t91159, t91160, t91162, t91167, t91171, t91179)
}
