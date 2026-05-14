//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 646/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk646<F: Float>(t6889: F, t7691: F, t6888: F, t1834: F, t225: F, t567: F, t214: F, t1985: F, t1842: F, t6906: F) -> (F, F, F, F, F, F) {
    let t7692 = t6889 * t7691;
    let t7693 = t6888 * t7692;
    let t7696 = t1834 * t225 * t567;
    let t7697 = t214 * t7696;
    let t7698 = t1985 * t7697;
    let t7700 = t6906 * t1842;
    (t7692, t7693, t7696, t7697, t7698, t7700)
}
