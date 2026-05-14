//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 703/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk703<F: Float>(t5: F, t1437: F, t2235: F, t2240: F, t3951: F, t3953: F, t3958: F, t4021: F, t605: F, t645: F, t86: F, t112: F, t111: F, t1441: F, t671: F, t89: F, t1266: F, t1458: F) -> (F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t4025 = piecewise3(t8, 0.0, -4.0 * t1437 * t2235 + 20.0 * t2240 * t3958 + t3951 * t86 - 4.0 * t3953 * t645 - 4.0 * t4021 * t605);
    let t4026 = t4025 * t112;
    let t4028 = t1441 * t111;
    let t4034 = t89 * t671;
    let t4037 = t1266 * t1458;
    (t4025, t4026, t4028, t4034, t4037)
}
