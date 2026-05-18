//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1143/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1143<F: Float>(t25: F, t6665: F, t28: F, t6504: F, t8307: F, t8513: F, t3701: F, t6995: F, t2314: F, t8327: F, t4034: F, t1266: F, t8326: F) -> (F, F, F, F, F, F, F) {
    let t30767 = t25 * t6665;
    let t30974 = t28 * t6665;
    let t31019 = t8513 * t8307 * t6504;
    let t31035 = t3701 * t6995;
    let t31054 = t2314 * t8327;
    let t31055 = F::new(2.0) * t31054;
    let t31056 = t4034 * t8327;
    let t31057 = F::new(2.0) * t31056;
    let t31058 = t1266 * t8326;
    (t30767, t30974, t31019, t31035, t31055, t31057, t31058)
}
