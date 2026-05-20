//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1399/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1399<F: Float>(t10216: F, t10969: F, t135: F, t4608: F, t973: F, t10868: F, t1539: F, t248: F, t1041: F, t1009: F, t4552: F, t1011: F) -> (F, F, F, F, F) {
    let t14187 = t10969 * t10216;
    let t14192 = t135 * t4608;
    let t14194 = t973 * t14192 / F::new(432.0);
    let t14202 = t248 * t10868 * t1539;
    let t14203 = t1041 * t14202;
    let t14205 = t4552 * t1009;
    let t14206 = t14205 * t1011;
    (t14187, t14194, t14203, t14205, t14206)
}
