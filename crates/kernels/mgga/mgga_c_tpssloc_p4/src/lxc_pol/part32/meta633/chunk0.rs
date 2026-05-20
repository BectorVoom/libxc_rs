//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2046/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2046<F: Float>(t87363: F, t242: F, t812: F, t81816: F, t25064: F, t81788: F, t25135: F, t838: F, t2693: F, t7503: F, t25132: F, t81882: F) -> (F, F, F, F, F, F) {
    let t87364 = F::new(7.0) / F::new(576.0) * t87363;
    let t87368 = t812 * t81816 * t242;
    let t87387 = t81788 * t25064;
    let t87401 = t25135 * t838;
    let t87402 = F::new(7.0) / F::new(1152.0) * t87401;
    let t87403 = t7503 * t2693;
    let t87405 = t81882 * t25132;
    (t87364, t87368, t87387, t87402, t87403, t87405)
}
