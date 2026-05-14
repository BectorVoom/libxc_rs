//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 845/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk845<F: Float>(t5520: F, t751: F, t5392: F, t2658: F, t5660: F, t870: F, t172: F, t5522: F, t763: F, t2752: F, t5664: F, t4101: F, t4205: F, t634: F, t638: F, t5575: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t16578 = t5520 * t751;
    let t16586 = t751 * t5392;
    let t16587 = t2658 * t16586;
    let t16606 = t5660 * t870;
    let t16616 = t5522 * t172;
    let t16617 = t16616 * t763;
    let t16625 = t5664 * t2752;
    let t16630 = t4205 * t4101;
    let t16637 = t634 * t5392;
    let t16649 = t638 * t5392;
    let t16673 = t5575 * t68;
    (t16578, t16586, t16587, t16606, t16616, t16617, t16625, t16630, t16637, t16649, t16673)
}
