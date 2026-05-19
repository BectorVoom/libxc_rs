//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1124/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1124<F: Float>(t300: F, t5769: F, t961: F, t2904: F, t5790: F, t952: F, t959: F, t14473: F, t1589: F, t4483: F, t4493: F, t4489: F) -> (F, F, F, F, F) {
    let t17934 = t300 * t5769;
    let t17936 = F::cast_from(0.5848223622634646207e0_f64) * t17934 * t961;
    let t17937 = t2904 * t5790;
    let t17938 = t17937 * t952;
    let t17940 = F::cast_from(0.11696447245269292414e1_f64) * t959 * t17938;
    let t17942 = F::cast_from(0.11696447245269292414e1_f64) * t14473 * t1589;
    let t17944 = F::cast_from(0.11696447245269292414e1_f64) * t4483 * t4493;
    let t17946 = F::cast_from(0.23392894490538584828e1_f64) * t4483 * t4489;
    (t17936, t17940, t17942, t17944, t17946)
}
