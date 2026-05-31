//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2544/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2544<F: Float>(t43959: F, t4786: F, t11424: F, t15051: F, t11185: F, t15061: F, t1117: F, t14914: F, t3264: F, t11350: F, t1682: F, t1136: F, t15118: F, t3332: F, t44131: F, t51453: F, t51456: F, t51459: F, t51463: F, t51466: F, t51470: F, t51472: F, t51474: F, t51476: F) -> (F, F, F, F, F) {
    let t51478 = F::cast_from(0.48245938496077605201e2_f64) * t43959 * t4786;
    let t51480 = F::cast_from(12.0_f64) * t11424 * t15051;
    let t51482 = F::cast_from(0.96491876992155210402e2_f64) * t11185 * t15061;
    let t51485 = F::cast_from(6.0_f64) * t3264 * t14914 * t1117;
    let t51486 = t11350 * t1682;
    let t51493 = t51453 + t51456 - t51459 - t51463 - t51466 - t51470 + t51472 - t51474 + t51476 - t51478 + t51480 - t51482 + t51485 + F::cast_from(0.6207121550312808036e4_f64) * t51486 * t44131 * t1136 - F::cast_from(6.0_f64) * t3332 * t15118 * t1136;
    (t51478, t51480, t51482, t51485, t51493)
}
