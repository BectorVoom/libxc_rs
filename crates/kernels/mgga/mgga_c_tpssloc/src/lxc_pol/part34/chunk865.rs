//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 865/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk865<F: Float>(t20760: F, t20761: F, t20765: F, t20766: F, t20768: F, t9724: F, t9780: F, t9789: F, t9793: F, t9797: F, t9863: F, t4205: F, t5597: F) -> (F, F) {
    let t20812 = t9724 + t9863 + t9780 - t20760 + t20761 + t20765 + t20766 + t20768 - t9789 + t9793 + t9797;
    let t20815 = F::new(12.0) * t4205 * t5597;
    (t20812, t20815)
}
