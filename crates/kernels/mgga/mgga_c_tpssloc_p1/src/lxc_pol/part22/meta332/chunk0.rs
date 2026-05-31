//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1521/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1521<F: Float>(t12283: F, t5259: F, t5293: F, t120: F, t5286: F, t5303: F, t1340: F, t16060: F, t3789: F, t5234: F, t3798: F) -> (F, F, F, F, F, F, F) {
    let t16239 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t12283 * t5259;
    let t16241 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t12283 * t5293;
    let t16242 = t120 * t5286;
    let t16269 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t12283 * t5303;
    let t16278 = t16060 * t1340;
    let t16285 = t5234 * t3789;
    let t16288 = t5234 * t3798;
    (t16239, t16241, t16242, t16269, t16278, t16285, t16288)
}
