//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 585/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk585<F: Float>(t35311: F, t490: F, t338: F, t830: F, t1330: F, t28: F, t7490: F, t7552: F, t1326: F, t2016: F, t7551: F, t302: F, t7350: F, t22: F, t4616: F, t2078: F, t26: F) -> (F, F, F, F, F, F, F, F) {
    let t35312 = t490 * t35311;
    let t35589 = t338 * t830;
    let t35613 = t28 * t1330;
    let t35620 = t7490 * t7552;
    let t35688 = t2016 * t7551 * t1326;
    let t35718 = t7350 * t302;
    let t35928 = t4616 * t22;
    let t35959 = t2078 * t26;
    (t35312, t35589, t35613, t35620, t35688, t35718, t35928, t35959)
}
