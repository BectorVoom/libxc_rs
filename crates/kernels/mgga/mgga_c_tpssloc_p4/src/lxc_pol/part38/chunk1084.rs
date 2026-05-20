//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1084/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1084<F: Float>(t13508: F, t959: F, t2944: F, t4483: F, t2940: F, t4493: F, t4351: F, t892: F, t914: F, t2837: F, t4354: F, t1543: F, t2841: F) -> (F, F, F, F, F, F) {
    let t13510 = F::cast_from(0.17315859105681463759e2_f64) * t959 * t13508;
    let t13512 = F::cast_from(0.11696447245269292414e1_f64) * t4483 * t2944;
    let t13514 = F::cast_from(0.11696447245269292414e1_f64) * t2940 * t4493;
    let t13515 = t4351 * t892;
    let t13517 = F::new(2.0) * t13515 * t914;
    let t13519 = F::new(1.0) * t4354 * t2837;
    let t13520 = t1543 * t2841;
    (t13510, t13512, t13514, t13517, t13519, t13520)
}
