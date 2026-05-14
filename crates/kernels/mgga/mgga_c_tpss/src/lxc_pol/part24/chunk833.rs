//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 833/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk833<F: Float>(t1688: F, t2056: F, t4347: F, t1165: F, t5531: F, t5512: F, t5514: F, t645: F, t1168: F, t196: F, t197: F) -> (F, F, F) {
    let t5697 = 2.0 * t2056 * t1688;
    let t5699 = 2.0 * t4347 * t1688;
    let t5701 = 2.0 * t1165 * t5531;
    let t5702 = 2.0 * t5514 * t645 + t5512 + t5697 + t5699 + t5701;
    let t5705 = t1168 * t196;
    let t5706 = t5705 * t197;
    (t5702, t5705, t5706)
}
