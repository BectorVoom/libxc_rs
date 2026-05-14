//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 933/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk933<F: Float>(t9214: F, t9216: F, t9218: F, t9220: F, t12560: F, t12561: F, t9225: F, t3951: F, t604: F, t1406: F, t2239: F, t1437: F, t2241: F, t4021: F, t645: F, t2307: F) -> (F, F, F, F, F, F) {
    let t12562 = 0.312e1 * t9214;
    let t12563 = 0.2312e3 * t9216;
    let t12564 = 0.3468e3 * t9218;
    let t12565 = 0.56952e3 * t9220;
    let t12566 = t12560 - t12561 + t12562 - t12563 + t12564 + t12565 - t9225;
    let t12568 = t3951 * t604;
    let t12571 = t1406 * t2239;
    let t12582 = t1437 * t2241;
    let t12585 = t4021 * t645;
    let t12588 = t1437 * t2307;
    (t12566, t12568, t12571, t12582, t12585, t12588)
}
