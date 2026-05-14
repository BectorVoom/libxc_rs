//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 918/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk918<F: Float>(t14781: F, t1147: F, t4832: F, t1687: F, t3400: F, t3375: F, t1128: F, t4794: F, t1675: F, t3356: F, t14722: F, t14704: F, t3331: F, t3403: F, t4857: F, t15026: F, t3623: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15094 = 0.27785333333333333334e0 * t14781;
    let t15121 = t4832 * t1147;
    let t15126 = t1687 * t3400;
    let t15136 = t1687 * t3375;
    let t15141 = t4794 * t1128;
    let t15146 = t1675 * t3356;
    let t15194 = 0.2283111111111111111e-1 * t14722;
    let t15195 = 0.11415555555555555555e-1 * t14704;
    let t15207 = t1675 * t3331;
    let t15218 = t4857 * t3403;
    let t15245 = t15026 * t3623;
    (t15094, t15121, t15126, t15136, t15141, t15146, t15194, t15195, t15207, t15218, t15245)
}
