//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1036/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1036<F: Float>(t1385: F, t31090: F, t22635: F, t1992: F, t1377: F, t2015: F, t1307: F, t22633: F, t794: F, t8454: F, t6897: F, t225: F, t567: F, t6955: F, t214: F, t1985: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31091 = t31090 * t1385;
    let t31092 = t22635 * t31091;
    let t31094 = 0.3289868133696452873e-1 * t1992 * t31092;
    let t31099 = t1377 * t2015;
    let t31100 = t31099 * t1307;
    let t31101 = t22635 * t31100;
    let t31103 = 0.3289868133696452873e-1 * t22633 * t31101;
    let t31104 = t794 * t8454;
    let t31106 = 0.82246703342411321825e-2 * t6897 * t31104;
    let t31108 = t6955 * t225 * t567;
    let t31109 = t214 * t31108;
    let t31111 = 0.16449340668482264365e-1 * t1985 * t31109;
    (t31091, t31092, t31094, t31099, t31100, t31101, t31103, t31104, t31106, t31108, t31109, t31111)
}
