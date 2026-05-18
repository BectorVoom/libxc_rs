//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 919/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk919<F: Float>(t76538: F, t1550: F, t7778: F, t8975: F, t15081: F, t68613: F, t2416: F, t7349: F, t28317: F, t3157: F, t14387: F, t14389: F, t14393: F, t14398: F, t14399: F, t14400: F, t15051: F, t15420: F, t15423: F, t15424: F, t15425: F, t15426: F, t15427: F, t15428: F, t15429: F) -> (F, F, F, F, F, F) {
    let t76539 = F::new(0.15965655602485078085e0) * t76538;
    let t76541 = t1550 * t7778 * t8975;
    let t76542 = F::new(0.15965655602485078085e0) * t76541;
    let t76545 = t68613 * t15081;
    let t76547 = t7349 * t2416;
    let t76550 = t28317 * t3157;
    let t76586 = -t15420 + t15423 - t15424 - t15425 + t15426 + t15427 - t15428 - t15429 + t15051 + t14387 - t14389 + t14393 + t14398 - t14399 + t14400;
    (t76539, t76542, t76545, t76547, t76550, t76586)
}
