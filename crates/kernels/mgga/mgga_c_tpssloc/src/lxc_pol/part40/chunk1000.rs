//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1000/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1000<F: Float>(t14722: F, t14704: F, t1667: F, t2403: F, t14720: F, t4775: F, t699: F, t4772: F, t1657: F, t3263: F, t1098: F, t4737: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14723 = F::new(4.0) / F::new(9.0) * t14722;
    let t14724 = F::new(2.0) / F::new(9.0) * t14704;
    let t14766 = t2403 * t1667;
    let t14768 = F::new(0.13418888888888888889e0) * t14720;
    let t14781 = t699 * t4775;
    let t14782 = F::new(0.22076e0) * t14781;
    let t14818 = t699 * t4772;
    let t14838 = t1657 * t3263;
    let t14845 = t4737 * t1098;
    (t14723, t14724, t14766, t14768, t14781, t14782, t14818, t14838, t14845)
}
