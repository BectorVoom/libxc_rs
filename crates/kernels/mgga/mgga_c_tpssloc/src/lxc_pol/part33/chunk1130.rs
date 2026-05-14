//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1130/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1130<F: Float>(t23012: F, t7485: F, t7489: F, t25245: F, t82031: F, t7529: F, t22690: F, t7520: F, t81573: F, t2627: F, t7510: F, t23030: F, t25258: F, t7524: F, t81612: F, t81613: F) -> (F, F, F, F, F, F, F, F) {
    let t86955 = t23012 * t7485;
    let t86991 = t23012 * t7489;
    let t87068 = t82031 * t25245;
    let t87080 = t23012 * t7529;
    let t87140 = t81573 * t22690 * t7520;
    let t87142 = t2627 * t7510;
    let t87155 = t23030 * t25258;
    let t87177 = t81612 * t81613 * t7524;
    (t86955, t86991, t87068, t87080, t87140, t87142, t87155, t87177)
}
