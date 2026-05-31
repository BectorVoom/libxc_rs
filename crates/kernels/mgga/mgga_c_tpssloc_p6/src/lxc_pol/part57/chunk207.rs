//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 207/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk207<F: Float>(t1039: F, t364: F, t354: F, t270: F, t283: F, t61: F, t225: F, t382: F) -> (F, F, F, F, F) {
    let t1040 = t364 * t1039;
    let t1041 = t354 * t1040;
    let t1043 = F::cast_from(1.0_f64) / t283 / t270;
    let t1044 = t61 * t1043;
    let t1052 = t382 * t225;
    (t1040, t1041, t1043, t1044, t1052)
}
