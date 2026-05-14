//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1121/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1121<F: Float>(t6604: F, t9971: F, t206: F, t22723: F, t268: F, t23163: F, t1879: F, t80845: F, t1906: F, t23229: F, t81715: F, t225: F, t23228: F, t6563: F, t81597: F, t1882: F, t81686: F, t9537: F) -> (F, F, F, F, F, F, F, F, F) {
    let t82018 = t6604 * t9971;
    let t82031 = t22723 * t206 * t268;
    let t82038 = t22723 * t23163;
    let t82045 = t80845 * t1879;
    let t82046 = t82045 * t1906;
    let t82047 = 0.27720185200590482541e0 * t82046;
    let t82069 = t81715 * t23229;
    let t82070 = 0.98696044010893586188e-1 * t82069;
    let t82074 = t23228 * t225;
    let t82122 = t81597 * t6563;
    let t82123 = 0.16220877603642232915e0 * t82122;
    let t82153 = t81686 * t9537 * t1882;
    (t82018, t82031, t82038, t82045, t82047, t82070, t82074, t82123, t82153)
}
