//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2140/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2140<F: Float>(t11282: F, t1687: F, t1682: F, t3357: F, t1694: F, t3401: F, t11420: F, t3312: F, t4737: F, t11419: F, t1675: F, t50826: F) -> (F, F, F, F, F, F, F) {
    let t51376 = t1687 * t11282;
    let t51382 = t3357 * t1682;
    let t51389 = t3401 * t1694;
    let t51392 = t11420 * t1682;
    let t51402 = t4737 * t3312;
    let t51427 = t1675 * t11419;
    let t51550 = F::cast_from(0.23744444444444444444e-1_f64) * t50826;
    (t51376, t51382, t51389, t51392, t51402, t51427, t51550)
}
