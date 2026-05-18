//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1014/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1014<F: Float>(t3698: F, t3701: F, t112: F, t3931: F, t111: F, t1395: F, t5107: F, t671: F, t1266: F, t4072: F, t1774: F, t2363: F) -> (F, F, F, F, F, F) {
    let t12477 = t3698 * t3701;
    let t12521 = t3931 * t112;
    let t12524 = t1395 * t111;
    let t12545 = t5107 * t671;
    let t12550 = t1266 * t4072;
    let t12557 = t1774 * t2363;
    (t12477, t12521, t12524, t12545, t12550, t12557)
}
