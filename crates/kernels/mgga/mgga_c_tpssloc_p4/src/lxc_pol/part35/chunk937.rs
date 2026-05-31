//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 937/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk937<F: Float>(t1437: F, t5445: F, t1864: F, t5398: F, t1426: F, t5392: F, t584: F, t9212: F) -> (F, F, F, F, F) {
    let t20204 = t1437 * t5445;
    let t20207 = t1864 * t5398;
    let t20210 = t5392 * t1426;
    let t20215 = -t584 - t9212;
    let t20216 = F::cast_from(6.0_f64) * t20215;
    (t20204, t20207, t20210, t20215, t20216)
}
