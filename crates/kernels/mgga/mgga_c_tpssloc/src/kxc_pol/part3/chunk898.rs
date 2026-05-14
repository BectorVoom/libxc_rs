//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 898/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk898<F: Float>(t12708: F, t33: F, t12649: F, t12653: F, t12656: F, t12662: F, t12665: F, t1427: F, t1434: F, t2255: F, t2304: F, t3962: F, t3968: F, t3998: F, t4018: F, t609: F, t642: F, t80: F) -> (F,) {
    let t12709 = t33 * t12708;
    let t12718 = -t2255 * t1434 / 6.0 - t12649 * t80 / 12.0 - t12653 * t80 / 6.0 - t12656 * t80 / 6.0 - t3962 * t642 / 6.0 - t12662 * t80 / 12.0 - t12665 * t80 / 6.0 - t3968 * t642 / 6.0 + t12709 * t80 / 24.0 + t3998 * t642 / 12.0 + t1427 * t2304 / 24.0 - t609 * t4018 / 6.0;
    (t12718,)
}
