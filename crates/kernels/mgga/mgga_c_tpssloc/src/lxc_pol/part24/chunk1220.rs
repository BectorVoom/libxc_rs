//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1220/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1220<F: Float>(t22979: F, t2597: F, t82150: F, t82154: F, t82156: F, t82161: F, t82165: F, t82169: F, t82172: F, t82174: F, t82179: F, t82182: F, t225: F, t23202: F, t6556: F, t81632: F) -> (F, F, F) {
    let t82186 = 0.11514538467937585055e0 * t82150 - t82154 - 0.24674011002723396548e-1 * t82156 + 0.9869604401089358619e-1 * t82161 - 0.82246703342411321825e-2 * t82165 - 0.16449340668482264365e-1 * t82169 + 0.24674011002723396548e-1 * t82172 + 0.23029076935875170111e0 * t82174 + 0.49348022005446793095e-1 * t82179 - 0.24674011002723396548e-1 * t82182 + 12.0 * t2597 * t22979;
    let t82197 = t23202 * t225;
    let t82209 = t81632 * t6556;
    (t82186, t82197, t82209)
}
