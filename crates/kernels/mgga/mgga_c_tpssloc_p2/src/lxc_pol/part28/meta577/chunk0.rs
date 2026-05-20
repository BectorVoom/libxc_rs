//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1860/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1860<F: Float>(t1516: F, t81763: F, t23083: F, t25094: F, t1510: F, t2379: F, t25119: F, t815: F, t2631: F, t47285: F, t6605: F, t9972: F) -> (F, F, F, F) {
    let t87345 = t81763 * t1516;
    let t87347 = t23083 * t25094;
    let t87351 = t25119 * t815 * t1510 * t2379;
    let t87355 = t6605 * t9972 * t47285 * t2631;
    (t87345, t87347, t87351, t87355)
}
