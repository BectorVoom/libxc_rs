//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1060/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1060<F: Float>(t31058: F, t652: F, t2314: F, t8323: F, t4034: F, t1873: F, t6862: F, t6517: F, t6535: F, t8526: F, t1874: F, t22461: F, t26103: F, t6525: F, t532: F, t8492: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31059 = t652 * t31058;
    let t31060 = 2.0 * t31059;
    let t31065 = t2314 * t8323;
    let t31067 = t4034 * t8323;
    let t31069 = t6862 * t1873;
    let t31070 = t652 * t31069;
    let t31072 = t6517 * t6535;
    let t31077 = 4.0 * t8526 * t6535;
    let t31078 = t22461 * t1874;
    let t31080 = t26103 * t1874;
    let t31082 = t6517 * t6525;
    let t31084 = t532 * t8492;
    (t31060, t31065, t31067, t31069, t31070, t31072, t31077, t31078, t31080, t31082, t31084)
}
