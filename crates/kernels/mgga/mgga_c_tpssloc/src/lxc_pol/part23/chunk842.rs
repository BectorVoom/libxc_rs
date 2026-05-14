//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 842/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk842<F: Float>(t5194: F, t782: F, t3732: F, t67: F, t792: F, t1799: F, t212: F, t12214: F, t131: F, t205: F, t12199: F, t5202: F, t12225: F, t2586: F, t2371: F, t5154: F) -> (F, F, F, F, F, F, F, F) {
    let t16081 = t782 * t5194;
    let t16093 = t3732 * t67;
    let t16094 = t792 * t16093;
    let t16095 = t212 * t1799;
    let t16100 = t12214 * t131;
    let t16101 = t205 * t16100;
    let t16108 = t12199 * t5202;
    let t16118 = t12225 * t16095;
    let t16119 = t2586 * t16118;
    let t16164 = t5154 * t2371;
    (t16081, t16094, t16095, t16101, t16108, t16118, t16119, t16164)
}
