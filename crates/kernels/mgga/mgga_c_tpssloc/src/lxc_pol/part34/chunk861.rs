//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 861/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk861<F: Float>(t16617: F, t12943: F, t16630: F, t12946: F, t145: F, t20741: F, t185: F, t4315: F, t5544: F, t1484: F, t16606: F, t193: F, t20753: F, t20756: F, t2522: F, t262: F, t4314: F, t9780: F, t9789: F, t9793: F, t9797: F, t9863: F) -> (F, F, F, F, F, F) {
    let t20760 = F::new(0.17544670867903938621e1) * t16617;
    let t20761 = F::new(0.35089341735807877242e1) * t12943;
    let t20765 = F::new(24.0) * t16630;
    let t20766 = F::new(12.0) * t12946;
    let t20767 = t145 * t20741;
    let t20768 = t20767 * t185;
    let t20769 = t4315 * t5544;
    let t20772 = F::new(9.0) * t1484 * t16606 * t2522 + F::new(6.0) * t193 * t20756 * t262 + F::new(18.0) * t20753 * t4314 + F::new(18.0) * t20769 * t4314 - t20760 + t20761 + t20765 + t20766 + t20768 + t9780 - t9789 + t9793 + t9797 + t9863;
    (t20760, t20761, t20765, t20766, t20768, t20772)
}
