//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1769/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1769<F: Float>(t6997: F, t7685: F, t1390: F, t5187: F, t6878: F, t1983: F, t192: F, t531: F, t1982: F) -> (F, F, F, F, F, F) {
    let t24989 = t7685 * t6997;
    let t24990 = t1390 * t5187;
    let t24991 = t6878 * t24990;
    let t24993 = F::new(3.0) * t1983 * t24991;
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    (t24989, t24990, t24991, t24993, t24994, t24995)
}
