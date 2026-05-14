//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1191/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1191<F: Float>(t17930: F, t44470: F, t19809: F, t821: F, t19817: F, t2162: F, t782: F, t818: F, t10592: F, t17964: F, t10596: F, t10795: F, t10799: F, t3678: F, t61033: F, t10805: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t63881 = t17930 * t44470;
    let t63884 = t19809 * t821;
    let t63885 = t19817 * t63884;
    let t63893 = t2162 * t818 * t782;
    let t63899 = t17964 * t10592;
    let t63901 = t17964 * t10596;
    let t63903 = t17964 * t10795;
    let t63905 = t17964 * t10799;
    let t63907 = t61033 * t3678;
    let t63909 = t17964 * t10805;
    (t63881, t63884, t63885, t63893, t63899, t63901, t63903, t63905, t63907, t63909)
}
