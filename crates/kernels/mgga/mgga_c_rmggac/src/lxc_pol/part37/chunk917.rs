//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 917/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk917<F: Float>(t76538: F, t1550: F, t7778: F, t8975: F, t15081: F, t68613: F, t2416: F, t7349: F, t28317: F, t3157: F, t73688: F, t73701: F) -> (F, F, F, F, F, F, F) {
    let t76539 = F::new(0.15965655602485078085e0) * t76538;
    let t76541 = t1550 * t7778 * t8975;
    let t76542 = F::new(0.15965655602485078085e0) * t76541;
    let t76545 = t68613 * t15081;
    let t76547 = t7349 * t2416;
    let t76550 = t28317 * t3157;
    let t76604 = F::new(0.5959043985061697516e-4) * t73688;
    let t76607 = F::new(0.2627895913935205078e-5) * t73701;
    (t76539, t76542, t76545, t76547, t76550, t76604, t76607)
}
