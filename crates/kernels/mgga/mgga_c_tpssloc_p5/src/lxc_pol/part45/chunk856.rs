//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 856/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk856<F: Float>(t31058: F, t652: F, t2015: F, t3886: F, t1385: F, t22635: F, t1992: F, t1377: F, t1307: F, t22633: F, t794: F, t8454: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31059 = t652 * t31058;
    let t31060 = F::new(2.0) * t31059;
    let t31090 = t3886 * t2015;
    let t31091 = t31090 * t1385;
    let t31092 = t22635 * t31091;
    let t31094 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t31092;
    let t31099 = t1377 * t2015;
    let t31100 = t31099 * t1307;
    let t31101 = t22635 * t31100;
    let t31103 = F::cast_from(0.3289868133696452873e-1_f64) * t22633 * t31101;
    let t31104 = t794 * t8454;
    (t31059, t31060, t31090, t31091, t31092, t31094, t31099, t31100, t31101, t31103, t31104)
}
