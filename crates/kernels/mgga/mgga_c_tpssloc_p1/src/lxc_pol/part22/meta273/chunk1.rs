//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1422/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1422<F: Float>(t12345: F, t1369: F, t241: F, t67: F, t6924: F, t1339: F, t2690: F, t1336: F) -> (F, F, F, F) {
    let t12346 = t12345 * t1369;
    let t12351 = t241 * t6924 * t67;
    let t12364 = t1339 * t2690;
    let t12365 = t1336 * t12364;
    (t12346, t12351, t12364, t12365)
}
