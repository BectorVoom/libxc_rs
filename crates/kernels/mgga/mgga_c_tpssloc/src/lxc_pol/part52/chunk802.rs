//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 802/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk802<F: Float>(t466: F, t8054: F, t1760: F, t2154: F, t3598: F, t1653: F, t7363: F, t7362: F, t1716: F, t2148: F, t1755: F, t7376: F, t7375: F, t1751: F, t2147: F, t462: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8055 = t466 * t8054;
    let t8060 = t2154 * t1760;
    let t8061 = t3598 * t8060;
    let t8066 = t7363 * t1653;
    let t8067 = t7362 * t8066;
    let t8070 = t1716 * t2148;
    let t8073 = t1755 * t7376;
    let t8074 = t7375 * t8073;
    let t8077 = t2147 * t1751;
    let t8078 = t462 * t8077;
    (t8055, t8061, t8066, t8067, t8070, t8073, t8074, t8077, t8078)
}
