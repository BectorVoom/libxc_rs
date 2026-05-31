//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1132/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1132<F: Float>(t10277: F, t3061: F, t14165: F, t4582: F, t12652: F, t4588: F, t12648: F, t10216: F, t10969: F, t135: F, t4608: F, t973: F) -> (F, F, F, F, F) {
    let t14172 = t3061 * t10277;
    let t14173 = t14172 * t14165;
    let t14174 = t4582 * t14173;
    let t14179 = t4588 * t12652;
    let t14180 = t4582 * t14179;
    let t14183 = t4588 * t12648;
    let t14184 = t4582 * t14183;
    let t14187 = t10969 * t10216;
    let t14188 = t14187 * t14165;
    let t14189 = t4582 * t14188;
    let t14192 = t135 * t4608;
    let t14194 = t973 * t14192 / F::cast_from(432.0_f64);
    (t14174, t14180, t14184, t14189, t14194)
}
