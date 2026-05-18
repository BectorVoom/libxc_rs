//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1247/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1247<F: Float>(t12240: F, t12251: F, t12255: F, t1336: F, t1352: F, t22709: F, t31201: F, t5334: F, t5344: F, t81184: F, t81187: F, t81189: F, t81193: F, t81197: F, t81199: F, t81203: F, t81209: F, t81213: F, t81216: F, t81218: F, t81222: F, t81225: F, t81230: F, t81234: F, t81238: F, t81243: F) -> F {
    let t81250 = -F::new(0.11514538467937585055e0) * t81184 - F::new(0.38381794893125283518e0) * t81187 + F::new(0.23029076935875170111e0) * t81189 + F::new(0.14804406601634037928e0) * t81193 + F::new(0.49348022005446793095e-1) * t81197 - F::new(3.0) * t1336 * t81199 * t1352 - F::new(3.0) * t5344 * t81203 * t1352 - F::new(0.49348022005446793095e-1) * t81209 - F::new(0.16449340668482264365e-1) * t81213 + F::new(0.24674011002723396548e-1) * t81216 + F::new(0.11514538467937585055e0) * t81218 - F::new(0.9869604401089358619e-1) * t81222 - F::new(0.24674011002723396548e-1) * t81225 - F::new(0.49348022005446793095e-1) * t81230 + F::new(0.9869604401089358619e-1) * t81234 + F::new(0.49348022005446793095e-1) * t81238 + F::new(6.0) * t5334 * t31201 * t12240 - F::new(6.0) * t1336 * t81243 * t12251 + F::new(6.0) * t1336 * t22709 * t12255;
    t81250
}
