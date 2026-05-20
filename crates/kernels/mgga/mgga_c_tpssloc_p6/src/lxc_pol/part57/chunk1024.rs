//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1024/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1024<F: Float>(t128570: F, t2020: F, t127114: F, t1983: F, t2095: F, t115925: F, t28831: F, t33363: F, t7756: F, t33623: F, t7685: F, t101138: F, t26161: F, t33221: F) -> (F, F, F, F, F, F) {
    let t128571 = t128570 * t2020;
    let t128573 = t1983 * t2095 * t127114;
    let t128575 = F::new(6.0) * t115925 * t28831;
    let t128577 = F::new(2.0) * t33363 * t7756;
    let t128581 = F::new(2.0) * t7685 * t33623;
    let t128584 = F::new(4.0) * t26161 * t101138 * t33221;
    (t128571, t128573, t128575, t128577, t128581, t128584)
}
