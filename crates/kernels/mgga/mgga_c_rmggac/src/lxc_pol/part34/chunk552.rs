//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 552/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk552<F: Float>(t15521: F, t1986: F, t2472: F, t675: F, t2471: F, t36: F, t739: F, t15281: F, t2211: F, t2367: F, t1356: F, t14451: F, t570: F, t5148: F, t551: F, t5259: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15522 = 0.42564599893297839398e-5 * t15521;
    let t15523 = t1986 * t2472;
    let t15524 = t675 * t15523;
    let t15525 = 0.42564599893297839398e-5 * t15524;
    let t15526 = t2471 * t36;
    let t15527 = t739 * t15526;
    let t15528 = 0.14967802127329760705e-1 * t15527;
    let t15529 = 0.14967802127329760705e-1 * t15281;
    let t15530 = t2211 * t2367;
    let t15531 = t1356 * t15530;
    let t15532 = 0.39914139006212695214e-1 * t15531;
    let t15533 = t14451 * t570;
    let t15534 = t5148 * t15533;
    let t15535 = 0.2993560425465952141e-1 * t15534;
    let t15536 = t14451 * t551;
    let t15537 = t5259 * t15536;
    (t15522, t15523, t15525, t15526, t15528, t15529, t15530, t15532, t15535, t15536, t15537)
}
