//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1180/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1180<F: Float>(t1100: F, t14758: F, t1667: F, t2403: F, t14720: F, t11215: F, t11217: F, t14722: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F) -> (F, F, F) {
    let t14759 = t1100 * t14758;
    let t14766 = t2403 * t1667;
    let t14768 = F::cast_from(0.13418888888888888889e0_f64) * t14720;
    let t14776 = -F::cast_from(0.11038e0_f64) * t11215 - F::cast_from(0.5519e-1_f64) * t11217 + F::cast_from(0.91983333333333333334e-1_f64) * t14766 + t14768 - F::cast_from(0.40256666666666666666e0_f64) * t14738 - F::cast_from(0.20128333333333333333e0_f64) * t14742 - F::cast_from(0.12077e1_f64) * t14733 + F::cast_from(0.12077e1_f64) * t14751 + F::cast_from(0.60385e0_f64) * t14755 + F::cast_from(0.181155e1_f64) * t14746 - F::cast_from(0.40256666666666666667e0_f64) * t14722;
    (t14759, t14766, t14776)
}
