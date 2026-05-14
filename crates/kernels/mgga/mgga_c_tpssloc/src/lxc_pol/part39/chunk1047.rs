//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1047/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1047<F: Float>(t1058: F, t11034: F, t11051: F, t11059: F, t11065: F, t14572: F, t14574: F, t14578: F, t14581: F, t14587: F, t14591: F, t14596: F, t14600: F, t14606: F, t14608: F, t1630: F, t1632: F, t3076: F, t3180: F, t3186: F, t3193: F, t3200: F, t3202: F, t4669: F, t4674: F, t4678: F, t4681: F) -> (F,) {
    let t14613 = t1058 * t14572 + 2.0 * t1058 * t14587 + t1058 * t14596 + t1058 * t14606 + 4.0 * t11034 * t4674 + t11051 * t1630 + 6.0 * t11059 * t14578 - 6.0 * t11065 * t14591 - 2.0 * t14574 * t3200 + 4.0 * t14581 * t3186 + 4.0 * t14600 * t3186 - t14608 * t3202 + t1632 * t3076 + 2.0 * t3180 * t4678 + 2.0 * t3180 * t4681 + 2.0 * t3193 * t4669;
    (t14613,)
}
