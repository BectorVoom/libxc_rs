//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 835/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk835<F: Float>(t194: F, t1979: F, t1982: F, t201: F, t5530: F, t2134: F, t27: F, t3118: F, t551: F, t2350: F, t4905: F, t26283: F) -> (F, F, F, F) {
    let t38780 = t194 * t5530 * t201 * t1979 * t1982;
    let t38784 = t2134 * t27 * t3118 * t551;
    let t38792 = t2350 * t4905;
    let t38793 = t26283 * t38792;
    (t38780, t38784, t38792, t38793)
}
