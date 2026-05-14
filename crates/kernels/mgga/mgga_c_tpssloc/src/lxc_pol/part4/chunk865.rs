//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 865/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk865<F: Float>(t14159: F, t973: F, t1043: F, t2770: F, t10277: F, t3061: F, t10216: F, t10969: F, t135: F, t4608: F, t10868: F, t1539: F, t248: F, t1041: F, t1009: F, t4552: F) -> (F, F, F, F, F, F, F) {
    let t14160 = t973 * t14159;
    let t14164 = t1043 * t2770;
    let t14172 = t3061 * t10277;
    let t14187 = t10969 * t10216;
    let t14192 = t135 * t4608;
    let t14194 = t973 * t14192 / 432.0;
    let t14202 = t248 * t10868 * t1539;
    let t14203 = t1041 * t14202;
    let t14205 = t4552 * t1009;
    (t14160, t14164, t14172, t14187, t14194, t14203, t14205)
}
