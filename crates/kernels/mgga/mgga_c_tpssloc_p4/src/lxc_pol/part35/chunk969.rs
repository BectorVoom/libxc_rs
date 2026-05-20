//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 969/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk969<F: Float>(t12044: F, t12046: F, t12048: F, t12053: F, t12055: F, t12057: F, t12059: F, t1297: F, t1390: F, t1799: F, t193: F, t20067: F, t20372: F, t20398: F, t20416: F, t20520: F, t20675: F, t3918: F, t533: F, t9780: F, t9789: F) -> F {
    let t20679 = t1390 * t193 * t20675 * t533 + F::new(3.0) * t1297 * t193 * t20416 + F::new(9.0) * t1799 * t20067 * t3918 - t12044 - t12046 - t12048 + t12053 - t12055 - t12057 - t12059 - t20372 + t20398 + t20520 + t9780 - t9789;
    t20679
}
