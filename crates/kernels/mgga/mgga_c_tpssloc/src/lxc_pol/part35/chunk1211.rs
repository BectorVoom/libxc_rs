//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1211/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1211<F: Float>(t12571: F, t27331: F, t2240: F, t29473: F, t33: F, t111: F, t29485: F, t112: F, t29865: F, t1851: F, t8119: F, t1858: F, t8110: F, t580: F, t2169: F, t6483: F) -> (F, F, F, F, F, F, F, F) {
    let t104958 = t12571 * t27331;
    let t104968 = t2240 * t33 * t29473;
    let t104990 = t29485 * t111;
    let t105105 = t29865 * t112;
    let t105131 = t1851 * t8119;
    let t105144 = t8110 * t1858;
    let t105146 = t29865 * t580;
    let t105147 = t2169 * t6483;
    (t104958, t104968, t104990, t105105, t105131, t105144, t105146, t105147)
}
