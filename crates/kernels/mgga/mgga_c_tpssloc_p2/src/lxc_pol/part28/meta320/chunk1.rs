//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1249/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1249<F: Float>(t11717: F, t1210: F, t11713: F, t248: F, t3509: F, t3570: F, t3506: F, t135: F, t3561: F, t1174: F, t3247: F, t415: F) -> (F, F, F, F) {
    let t11737 = t1210 * t11717;
    let t11738 = t11713 * t11737;
    let t11745 = t248 * t3570 * t3509;
    let t11746 = t3506 * t11745;
    let t11754 = t135 * t3561;
    let t11755 = t1174 * t11754;
    let t11778 = F::new(1.0) / t415 / t3247;
    (t11738, t11746, t11755, t11778)
}
