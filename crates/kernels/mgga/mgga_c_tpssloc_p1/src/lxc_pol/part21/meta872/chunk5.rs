//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3217/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3217<F: Float>(t55998: F, t56034: F, t56075: F, t66935: F, t1395: F, t671: F, t112: F, t20148: F, t12524: F, t12813: F, t1401: F, t1458: F, t16521: F, t16524: F, t16538: F, t16541: F, t19534: F, t20162: F, t20173: F, t20176: F, t2363: F, t3938: F, t3941: F, t4072: F, t5456: F, t55568: F, t55571: F, t577: F) -> (F, F) {
    let t66937 = t55998 + t56034 + t56075 + t66935;
    let t66940 = t1395 * t671;
    let t66958 = t20148 * t112;
    let t66961 = F::new(108.0) * t16524 * t16538 + F::new(0.135e2) * t1401 * t55568 + F::new(27.0) * t55571 * t5456 + F::new(0.45e1) * t66937 * t577 + F::new(54.0) * t66940 * t5456 + F::new(54.0) * t16521 * t4072 + F::new(0.135e2) * t20162 * t2363 + F::new(54.0) * t3941 * t1458 * t12813 + F::new(27.0) * t3938 * t19534 + F::new(54.0) * t16524 * t16541 + F::new(108.0) * t20173 * t20176 + F::new(108.0) * t12524 * t20176 + F::new(27.0) * t66958 * t671;
    (t66937, t66961)
}
