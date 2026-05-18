//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 965/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk965<F: Float>(t3749: F, t9577: F, t3726: F, t3745: F, t1314: F, t2566: F, t3741: F, t3732: F, t792: F, t118: F, t3734: F, t794: F) -> (F, F, F, F, F, F) {
    let t12196 = F::new(0.99999999999999999997e-2) * t9577 * t3749;
    let t12197 = t3726 * t3745;
    let t12199 = t2566 * t1314;
    let t12200 = t12199 * t3741;
    let t12202 = t792 * t3732;
    let t12204 = t118 * t794 * t3734;
    (t12196, t12197, t12199, t12200, t12202, t12204)
}
