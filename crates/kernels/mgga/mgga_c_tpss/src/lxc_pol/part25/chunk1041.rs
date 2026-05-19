//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1041/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1041<F: Float>(t14219: F, t14250: F, t14320: F, t14347: F, t219: F, t4779: F, t4783: F, t818: F, t8348: F, t1395: F, t2406: F, t3721: F, param_beta: F) -> (F, F, F, F, F) {
    let t14349 = t14219 + t14250 + t14320 + t14347;
    let t14350 = param_beta * t14349;
    let t14352 = t4779 * t219;
    let t14363 = t8348 * t4783 * t818;
    let t14367 = t2406 * t1395 * t3721;
    (t14349, t14350, t14352, t14363, t14367)
}
