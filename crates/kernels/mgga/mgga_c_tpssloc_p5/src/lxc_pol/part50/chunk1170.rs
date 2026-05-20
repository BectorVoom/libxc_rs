//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1170/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1170<F: Float>(t234: F, t240: F, t241: F, t4248: F, t776: F, t812: F, t9646: F, t4234: F, t6605: F, t6612: F, t25119: F, t4255: F, t6619: F) -> (F, F, F) {
    let t118546 = t812 * t234 * t240 * t241 * t9646 * t4248 * t776;
    let t118549 = t6605 * t6612 * t4234;
    let t118552 = t25119 * t6619 * t4255;
    (t118546, t118549, t118552)
}
