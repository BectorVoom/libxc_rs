//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2330/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2330<F: Float>(t7365: F, t94490: F, t1715: F, t974: F, t24847: F, t24771: F, t7999: F, t15418: F, t2127: F, t221: F, t27553: F, t11877: F, t11907: F, t11914: F, t15245: F, t15429: F, t24765: F, t24834: F, t24838: F, t27406: F, t27454: F, t27546: F, t7283: F, t8082: F, t8083: F, t86073: F, t86095: F, t94588: F) -> (F, F) {
    let t95758 = t94490 * t7365;
    let t95760 = t974 * t1715;
    let t95761 = t24847 * t95760;
    let t95768 = t7999 * t24771;
    let t95772 = t2127 * t221 * t15418;
    let t95774 = F::cast_from(0.24369393582936687948e-2_f64) * t95772 * t27553;
    let t95779 = -F::cast_from(0.18277045187202515961e-2_f64) * t86073 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t94588 * t27454 + F::cast_from(0.48738787165873375896e-2_f64) * t95758 - F::cast_from(0.16449340668482264365e-1_f64) * t95761 * t24834 - t15245 * t24838 + t11877 * t8083 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t24765 + F::cast_from(0.48738787165873375895e-2_f64) * t95768 - F::cast_from(0.18277045187202515961e-2_f64) * t86095 + t95774 - F::new(2.0) * t11907 * t27546 + t11914 * t8082 * t15429;
    (t95772, t95779)
}
