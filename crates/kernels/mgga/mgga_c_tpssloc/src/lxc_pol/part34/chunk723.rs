//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 723/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk723<F: Float>(t3865: F, t5234: F, t12189: F, t1811: F, t1815: F, t3862: F, t3802: F, t1834: F, t3787: F, t111: F, t1851: F, t5520: F, t751: F, t5392: F, t2658: F, t5660: F, t870: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16336 = t5234 * t3865;
    let t16341 = t12189 * t1811;
    let t16350 = t1815 * t3862;
    let t16394 = t5234 * t3802;
    let t16428 = t3787 * t1834;
    let t16524 = t1851 * t111;
    let t16578 = t5520 * t751;
    let t16586 = t751 * t5392;
    let t16587 = t2658 * t16586;
    let t16606 = t5660 * t870;
    (t16336, t16341, t16350, t16394, t16428, t16524, t16578, t16587, t16606)
}
