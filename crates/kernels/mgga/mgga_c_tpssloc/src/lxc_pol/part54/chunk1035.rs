//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1035/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1035<F: Float>(t2314: F, t8327: F, t4034: F, t1266: F, t8326: F, t652: F, t2015: F, t3886: F) -> (F, F, F, F, F) {
    let t31054 = t2314 * t8327;
    let t31055 = 2.0 * t31054;
    let t31056 = t4034 * t8327;
    let t31057 = 2.0 * t31056;
    let t31058 = t1266 * t8326;
    let t31059 = t652 * t31058;
    let t31060 = 2.0 * t31059;
    let t31090 = t3886 * t2015;
    (t31055, t31057, t31058, t31060, t31090)
}
