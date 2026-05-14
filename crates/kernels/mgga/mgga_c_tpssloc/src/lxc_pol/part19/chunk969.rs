//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 969/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk969<F: Float>(t11882: F, t11883: F, t10471: F, t3502: F, t11712: F, t3508: F, t6739: F, t11624: F, t3612: F, t1215: F, t3590: F, t1246: F, t11707: F, t3609: F, t3623: F, t3620: F, t5079: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11884 = t11882 * t11883;
    let t11887 = t10471 * t3502;
    let t11888 = t11712 * t11887;
    let t11889 = t6739 * t3508;
    let t11890 = t11882 * t11889;
    let t11893 = t11624 * t3612;
    let t11896 = t3590 * t1215;
    let t11897 = t11896 * t1246;
    let t11904 = t11707 * t3609;
    let t11907 = t11707 * t3623;
    let t11910 = t3620 * t5079;
    (t11884, t11887, t11888, t11889, t11890, t11893, t11897, t11904, t11907, t11910)
}
