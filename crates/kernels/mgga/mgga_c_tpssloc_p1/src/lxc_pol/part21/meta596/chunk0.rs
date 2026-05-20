//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2347/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2347<F: Float>(t3014: F, t343: F, t3475: F, t460: F, t253: F, t254: F, t4540: F, t382: F, t1458: F, t649: F, t1453: F, t666: F) -> (F, F, F, F, F, F, F) {
    let t23547 = t3014 * t343;
    let t24705 = t3475 * t460;
    let t25168 = t253 * t254;
    let t25608 = t4540 * t343;
    let t25757 = t382 * t254;
    let t26114 = t649 * t1458;
    let t26129 = t1453 * t666;
    (t23547, t24705, t25168, t25608, t25757, t26114, t26129)
}
