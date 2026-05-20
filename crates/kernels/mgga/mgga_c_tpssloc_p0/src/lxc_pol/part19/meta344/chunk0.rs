//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1230/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1230<F: Float>(t41274: F, t185: F, t39110: F, t707: F, t2447: F, t32: F, t2659: F, t9929: F, t9932: F, t31: F, t717: F, t9898: F) -> (F, F, F, F, F) {
    let t41275 = F::cast_from(0.70178683471615754484e1_f64) * t41274;
    let t41278 = F::new(4.0) * t707 * t185 * t39110;
    let t41279 = t32 * t2447;
    let t41281 = F::new(72.0) * t41279 * t2659;
    let t41282 = t9929 * t9932;
    let t41283 = F::new(144.0) * t41282;
    let t41284 = t31 * t717;
    let t41286 = F::new(96.0) * t41284 * t9898;
    (t41275, t41278, t41281, t41283, t41286)
}
