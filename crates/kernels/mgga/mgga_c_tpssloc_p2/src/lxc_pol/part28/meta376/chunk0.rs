//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1440/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1440<F: Float>(t1100: F, t14758: F, t1667: F, t2403: F, t14720: F, t11215: F, t11217: F, t14722: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F) -> (F, F, F) {
    let t14759 = t1100 * t14758;
    let t14766 = t2403 * t1667;
    let t14768 = F::cast_from(0.13418888888888888889e0_f64) * t14720;
    let t14776 = -F::new(0.11038e0) * t11215 - F::new(0.5519e-1) * t11217 + F::cast_from(0.91983333333333333334e-1_f64) * t14766 + t14768 - F::cast_from(0.40256666666666666666e0_f64) * t14738 - F::cast_from(0.20128333333333333333e0_f64) * t14742 - F::new(0.12077e1) * t14733 + F::new(0.12077e1) * t14751 + F::new(0.60385e0) * t14755 + F::new(0.181155e1) * t14746 - F::cast_from(0.40256666666666666667e0_f64) * t14722;
    (t14759, t14766, t14776)
}
