//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1244/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1244<F: Float>(t22723: F, t22891: F, t22920: F, t12437: F, t1985: F, t6889: F, t6906: F, t22666: F, t22685: F, t22686: F, t117: F, t5247: F, t6559: F) -> (F, F, F, F, F) {
    let t80670 = t22723 * t22891;
    let t80671 = t80670 * t22920;
    let t80675 = t1985 * t6889 * t6906 * t12437;
    let t80678 = t22685 * t22666 * t22686;
    let t80681 = t6559 * t5247 * t117;
    (t80670, t80671, t80675, t80678, t80681)
}
