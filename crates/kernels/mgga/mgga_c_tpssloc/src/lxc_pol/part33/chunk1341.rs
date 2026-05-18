//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1341/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1341<F: Float>(t105776: F, t105829: F, t1634: F, t5392: F, t1052: F, t1599: F, t23327: F, t23329: F, t25442: F, t28474: F, t28515: F, t28678: F, t28697: F, t28701: F, t3174: F, t4660: F, t6687: F, t7553: F, t82342: F, t88050: F, t99131: F, t99151: F, t99184: F, t99190: F, t99273: F, t99336: F) -> (F, F, F) {
    let t105830 = t105776 + t105829;
    let t105840 = t5392 * t1634;
    let t105863 = -F::new(0.82246703342411321826e-2) * t23327 * t99336 * t7553 - F::new(0.16449340668482264365e-1) * t23327 * t88050 * t28701 - F::new(0.82246703342411321826e-2) * t23327 * t99273 * t7553 + F::new(0.16449340668482264365e-1) * t23327 * t23329 * t82342 * t105840 + F::new(6.0) * t1052 * t3174 * t28678 * t1634 - F::new(0.82246703342411321826e-2) * t99151 - F::new(0.24674011002723396548e-1) * t6687 * t1599 * t28474 - F::new(0.82246703342411321826e-2) * t23327 * t25442 * t28515 - F::new(0.82246703342411321826e-2) * t99184 - F::new(0.54831135561607547883e-2) * t99190 - F::new(0.16449340668482264365e-1) * t23327 * t99131 * t7553 - F::new(18.0) * t4660 * t28697;
    (t105830, t105840, t105863)
}
