//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2436/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2436<F: Float>(t10771: F, t14271: F, t14276: F, t17535: F, t17538: F, t17541: F, t21259: F, t2886: F, t4433: F, t49430: F, t5743: F, t69288: F, t69291: F, t69294: F, t69297: F, t69299: F, t69302: F, t69305: F, t69307: F, t69310: F, t69313: F, t931: F) -> F {
    let t69326 = t69288 + t69291 - t69294 - t69297 + t69299 + t69302 + t69305 - t69307 - t69310 - t69313 + F::cast_from(18.0_f64) * t14271 * t17535 - F::cast_from(12.0_f64) * t14276 * t17538 - F::cast_from(0.57895126195293126241e3_f64) * t49430 * t17541 - F::cast_from(24.0_f64) * t10771 * t21259 * t931 + F::cast_from(18.0_f64) * t2886 * t5743 * t4433;
    t69326
}
