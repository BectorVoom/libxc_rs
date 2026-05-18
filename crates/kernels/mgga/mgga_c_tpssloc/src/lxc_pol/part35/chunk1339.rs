//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1339/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1339<F: Float>(t1222: F, t29601: F, t1193: F, t29585: F, t2136: F, t29562: F, t52: F, t27674: F, t5040: F, t1409: F, t8027: F, t29643: F, t3503: F, t86264: F) -> (F, F, F, F, F, F) {
    let t104128 = t29601 * t1222;
    let t104139 = t29585 * t1193;
    let t104142 = t29562 * t52 * t2136;
    let t104150 = t27674 * t5040;
    let t104153 = t8027 * t1409 * t2136;
    let t104181 = t86264 * t3503 * t29643;
    (t104128, t104139, t104142, t104150, t104153, t104181)
}
