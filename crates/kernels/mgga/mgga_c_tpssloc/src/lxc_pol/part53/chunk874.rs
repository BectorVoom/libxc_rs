//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 874/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk874<F: Float>(t22674: F, t8621: F, t6897: F, t6906: F, t7213: F, t6889: F, t1985: F, t2085: F, t214: F) -> (F, F, F, F, F, F) {
    let t31594 = t22674 * t8621;
    let t31595 = t6897 * t31594;
    let t31607 = t6906 * t7213;
    let t31608 = t6889 * t31607;
    let t31609 = t1985 * t31608;
    let t31611 = t214 * t2085;
    (t31594, t31595, t31607, t31608, t31609, t31611)
}
