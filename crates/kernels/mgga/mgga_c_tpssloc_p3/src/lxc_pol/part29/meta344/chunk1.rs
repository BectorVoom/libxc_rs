//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1408/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1408<F: Float>(t3792: F, t3850: F, t1337: F, t550: F, t1338: F, t3879: F, t3773: F, t68: F) -> (F, F, F, F, F) {
    let t12240 = t3792 * t3850;
    let t12247 = t1337 * t1337;
    let t12248 = F::new(1.0) / t12247;
    let t12250 = t3792 * t550;
    let t12259 = t1338 * t3879;
    let t12267 = t3773 * t68;
    (t12240, t12248, t12250, t12259, t12267)
}
