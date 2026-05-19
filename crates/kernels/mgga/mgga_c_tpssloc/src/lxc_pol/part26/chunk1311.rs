//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1311/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1311<F: Float>(t22986: F, t23270: F, t2553: F, t857: F, t865: F, t23196: F, t23204: F, t6562: F, t22979: F, t2597: F, t82150: F, t82154: F, t82156: F, t82161: F, t82165: F, t82169: F, t82172: F, t82174: F) -> F {
    let t82179 = t22986 * t23270 * t857 * t2553 * t865;
    let t82182 = t6562 * t23204 * t23196;
    let t82186 = F::cast_from(0.11514538467937585055e0_f64) * t82150 - t82154 - F::cast_from(0.24674011002723396548e-1_f64) * t82156 + F::cast_from(0.9869604401089358619e-1_f64) * t82161 - F::cast_from(0.82246703342411321825e-2_f64) * t82165 - F::cast_from(0.16449340668482264365e-1_f64) * t82169 + F::cast_from(0.24674011002723396548e-1_f64) * t82172 + F::cast_from(0.23029076935875170111e0_f64) * t82174 + F::cast_from(0.49348022005446793095e-1_f64) * t82179 - F::cast_from(0.24674011002723396548e-1_f64) * t82182 + F::new(12.0) * t2597 * t22979;
    t82186
}
