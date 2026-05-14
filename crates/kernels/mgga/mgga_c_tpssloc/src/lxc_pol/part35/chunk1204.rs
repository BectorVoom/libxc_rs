//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1204/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1204<F: Float>(t24746: F, t27607: F, t8027: F, t1409: F, t1714: F, t2132: F, t52: F, t6138: F, t1222: F, t29597: F, t29601: F, t1193: F, t29585: F, t2136: F, t29562: F, t27674: F, t5040: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t104111 = t8027 * t27607 * t24746;
    let t104118 = t1409 * t1714;
    let t104120 = t2132 * t104118 * t24746;
    let t104122 = t52 * t6138;
    let t104124 = t2132 * t104122 * t24746;
    let t104126 = t29597 * t1222;
    let t104128 = t29601 * t1222;
    let t104139 = t29585 * t1193;
    let t104142 = t29562 * t52 * t2136;
    let t104150 = t27674 * t5040;
    (t104111, t104118, t104120, t104122, t104124, t104126, t104128, t104139, t104142, t104150)
}
