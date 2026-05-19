//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 588/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk588<F: Float>(t300: F, t4479: F, t4447: F, t1573: F, t961: F, t1589: F, t2940: F, t1580: F, t2904: F, t952: F, t959: F, t4471: F, t942: F, t951: F) -> (F, F, F, F, F, F) {
    let t4480 = t300 * t4479;
    let t4482 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t4447;
    let t4483 = t300 * t1573;
    let t4485 = F::cast_from(0.5848223622634646207e0_f64) * t4483 * t961;
    let t4487 = F::cast_from(0.5848223622634646207e0_f64) * t2940 * t1589;
    let t4488 = t2904 * t1580;
    let t4489 = t4488 * t952;
    let t4491 = F::cast_from(0.11696447245269292414e1_f64) * t959 * t4489;
    let t4493 = t942 * t4471 * t951;
    (t4480, t4482, t4485, t4487, t4491, t4493)
}
