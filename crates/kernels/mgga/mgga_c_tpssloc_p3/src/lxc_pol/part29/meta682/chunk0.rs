//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2304/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2304<F: Float>(t24574: F, t27462: F, t1185: F, t86036: F, t974: F, t3030: F, t460: F, t27488: F, t27491: F, t27495: F, t27497: F, t1170: F, t2121: F, t27732: F) -> (F, F, F, F, F) {
    let t95192 = F::cast_from(0.18277045187202515961e-2_f64) * t24574 * t27462;
    let t95194 = t86036 * t974 * t1185;
    let t95195 = t460 * t3030;
    let t95197 = t95195 * t27488 * t27491;
    let t95201 = t95195 * t27495 * t27497;
    let t95213 = F::cast_from(0.54831135561607547884e-2_f64) * t2121 * t1170 * t27732;
    (t95192, t95194, t95197, t95201, t95213)
}
