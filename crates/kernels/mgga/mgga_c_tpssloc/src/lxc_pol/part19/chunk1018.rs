//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1018/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1018<F: Float>(t16046: F, t544: F, t1332: F, t5333: F, t5194: F, t782: F, t3732: F, t67: F, t792: F, t12214: F, t131: F, t205: F, t1345: F, t68: F, t12418: F, t820: F) -> (F, F, F, F, F, F, F) {
    let t16047 = t544 * t16046;
    let t16055 = t1332 * t5333;
    let t16081 = t782 * t5194;
    let t16093 = t3732 * t67;
    let t16094 = t792 * t16093;
    let t16100 = t12214 * t131;
    let t16101 = t205 * t16100;
    let t16186 = t1345 * t68;
    let t16224 = t12418 * t820;
    (t16047, t16055, t16081, t16094, t16101, t16186, t16224)
}
