//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1242/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1242<F: Float>(t12022: F, t1985: F, t6889: F, t80640: F, t1372: F, t794: F, t6897: F, t6907: F, t213: F, t225: F, t22633: F, t22637: F) -> (F, F, F, F) {
    let t80643 = t1985 * t6889 * t80640 * t12022;
    let t80645 = t794 * t1372;
    let t80647 = t6897 * t80645 * t6907;
    let t80650 = t213 * t1372 * t225;
    let t80652 = t22633 * t80650 * t22637;
    (t80643, t80645, t80647, t80652)
}
