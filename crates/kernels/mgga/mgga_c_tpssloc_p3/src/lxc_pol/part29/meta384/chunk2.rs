//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1560/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1560<F: Float>(t225: F, t4553: F, t1634: F, t3206: F, t3174: F, t4559: F, t4555: F, t4657: F, t990: F, t14488: F, t381: F, t1060: F) -> (F, F, F, F, F, F) {
    let t14545 = t4553 * t225;
    let t14548 = t1634 * t3206;
    let t14549 = t3174 * t14548;
    let t14552 = t4559 * t225;
    let t14555 = t4555 * t225;
    let t14562 = t990 * t4657;
    let t14571 = t381 * t14488;
    let t14572 = t14571 * t1060;
    (t14545, t14549, t14552, t14555, t14562, t14572)
}
