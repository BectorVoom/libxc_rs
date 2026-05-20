//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1064/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1064<F: Float>(t6614: F, t831: F, t1899: F, t838: F, t234: F, t59: F, t240: F) -> (F, F, F, F) {
    let t6615 = t6614 * t831;
    let t6617 = t1899 * t838;
    let t6619 = t234 * t59;
    let t6620 = t6619 * t240;
    (t6615, t6617, t6619, t6620)
}
