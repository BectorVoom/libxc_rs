//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 428/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk428<F: Float>(t1025: F, t1509: F, t1032: F, t1038: F, t1501: F, t141: F, t1030: F, t1037: F, t1503: F, t1043: F) -> (F, F, F, F, F, F) {
    let t1510 = t1025 * t1509;
    let t1513 = t1032 * t1509;
    let t1515 = t1038 * t1501;
    let t1516 = t141 * t1515;
    let t1518 = 0.1898925e1 * t1510 - t1030 + 0.29896666666666666667e0 * t1503 + 0.3071625e0 * t1513 - t1037 + 0.82156666666666666667e-1 * t1516;
    let t1519 = t1518 * t1043;
    (t1510, t1513, t1515, t1516, t1518, t1519)
}
