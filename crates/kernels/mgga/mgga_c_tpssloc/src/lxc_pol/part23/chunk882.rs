//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 882/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk882<F: Float>(t20217: F, t31: F, t65: F, t1426: F, t5399: F, t1410: F, t5427: F, t1409: F, t5392: F) -> (F, F, F, F, F) {
    let t20218 = t31 * t20217;
    let t20219 = t20218 * t65;
    let t20222 = t5399 * t1426;
    let t20227 = t1410 * t5427;
    let t20234 = t5392 * t1409;
    (t20218, t20219, t20222, t20227, t20234)
}
