//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2101/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2101<F: Float>(t27498: F, t85853: F, t27533: F, t86094: F, t24826: F, t27521: F, t24574: F, t27462: F, t3030: F, t460: F, t27488: F, t27491: F) -> (F, F, F, F, F, F) {
    let t95136 = F::cast_from(0.54831135561607547884e-2_f64) * t85853 * t27498;
    let t95163 = F::cast_from(0.18277045187202515961e-2_f64) * t86094 * t27533;
    let t95165 = F::cast_from(0.54831135561607547884e-2_f64) * t24826 * t27521;
    let t95192 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27462;
    let t95195 = t460 * t3030;
    let t95197 = t95195 * t27488 * t27491;
    (t95136, t95163, t95165, t95192, t95195, t95197)
}
