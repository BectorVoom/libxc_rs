//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1690/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1690<F: Float>(t20118: F, t20147: F, t3: F, t112: F, t6470: F, t576: F, t671: F, t1458: F, t4072: F, t5493: F, t12524: F, t1401: F, t16521: F, t16524: F, t19534: F, t3938: F, t3941: F, t5371: F, t5376: F, t5456: F, t577: F) -> (F, F, F, F, F, F, F) {
    let t20148 = t20118 + t20147;
    let t20149 = t3 * t20148;
    let t20162 = t6470 * t112;
    let t20173 = t576 * t671;
    let t20176 = t1458 * t4072;
    let t20181 = t5493 * t671;
    let t20186 = F::new(0.45e1) * t20148 * t577 + F::new(0.135e2) * t20162 * t671 + F::new(27.0) * t16521 * t1458 + F::new(54.0) * t16524 * t5376 + F::new(27.0) * t5371 * t4072 + F::new(27.0) * t12524 * t5456 + F::new(27.0) * t20173 * t5456 + F::new(54.0) * t3941 * t20176 + F::new(0.135e2) * t3938 * t5493 + F::new(27.0) * t3941 * t20181 + F::new(0.135e2) * t1401 * t19534;
    (t20148, t20149, t20162, t20173, t20176, t20181, t20186)
}
