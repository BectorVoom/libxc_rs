//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1127/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1127<F: Float>(t12273: F, t1992: F, t6976: F, t268: F, t547: F, t6559: F, t22705: F, t22733: F, t22633: F, t22694: F, t3807: F, t12272: F, t12248: F, t2006: F, t12240: F, t12251: F, t12255: F, t1336: F, t1352: F, t22709: F, t31201: F, t5334: F, t5344: F, t81184: F, t81187: F, t81189: F, t81193: F, t81197: F, t81199: F, t81203: F, t81209: F, t81213: F, t81216: F, t81218: F, t81222: F) -> (F, F) {
    let t81225 = t1992 * t6976 * t12273;
    let t81228 = t6559 * t547 * t268;
    let t81230 = t81228 * t22705 * t22733;
    let t81234 = t22633 * t6976 * t22694 * t3807;
    let t81238 = t22633 * t6976 * t12272 * t3807;
    let t81243 = t12248 * t2006;
    let t81250 = -0.11514538467937585055e0 * t81184 - 0.38381794893125283518e0 * t81187 + 0.23029076935875170111e0 * t81189 + 0.14804406601634037928e0 * t81193 + 0.49348022005446793095e-1 * t81197 - 3.0 * t1336 * t81199 * t1352 - 3.0 * t5344 * t81203 * t1352 - 0.49348022005446793095e-1 * t81209 - 0.16449340668482264365e-1 * t81213 + 0.24674011002723396548e-1 * t81216 + 0.11514538467937585055e0 * t81218 - 0.9869604401089358619e-1 * t81222 - 0.24674011002723396548e-1 * t81225 - 0.49348022005446793095e-1 * t81230 + 0.9869604401089358619e-1 * t81234 + 0.49348022005446793095e-1 * t81238 + 6.0 * t5334 * t31201 * t12240 - 6.0 * t1336 * t81243 * t12251 + 6.0 * t1336 * t22709 * t12255;
    (t81228, t81250)
}
