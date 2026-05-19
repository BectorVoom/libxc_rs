//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 840/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk840<F: Float>(t9871: F, t181: F, t686: F, t781: F, t756: F, t9727: F, t9780: F, t9789: F, t9793: F, t9797: F, t9863: F, t9865: F, t9867: F, t9870: F) -> (F, F, F, F) {
    let t9872 = F::cast_from(0.73245789224026180216e-3_f64) * t9871;
    let t9874 = t686 * t781 * t181;
    let t9876 = F::cast_from(0.56968947174242584612e-3_f64) * t756 * t9874;
    let t9877 = t9727 + t9863 + t9780 + t9865 - t9867 - t9789 + t9870 + t9872 + t9793 + t9797 - t9876;
    (t9872, t9874, t9876, t9877)
}
