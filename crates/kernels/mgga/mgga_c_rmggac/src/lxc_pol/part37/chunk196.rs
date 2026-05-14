//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 196/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk196<F: Float>(t1417: F, t385: F, t578: F, t1020: F, t1031: F, t1011: F, t1014: F, t1027: F, t1029: F, t1044: F, t1374: F, t1392: F, t1414: F, t1416: F, t436: F, t948: F, t975: F, t982: F) -> (F, F, F, F, F) {
    let t1418 = 4.0 * t1417;
    let t1419 = t385 * t578;
    let t1420 = 4.0 * t1419;
    let t1421 = 4.0 * t1020;
    let t1422 = 4.0 * t1031;
    let t1423 = t948 - t975 - t1374 + 0.93273e-1 * t436 * t1392 + t1414 + t1416 + t1418 - t1420 + t982 + t1011 - t1014 - t1421 - t1027 + t1029 - t1422 - t1044;
    (t1418, t1420, t1421, t1422, t1423)
}
