//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1227/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1227<F: Float>(t11172: F, t11616: F, t1244: F, t1246: F, t2152: F, t24667: F, t24841: F, t24849: F, t24852: F, t3493: F, t3604: F, t3611: F, t3624: F, t3625: F, t470: F, t493: F, t7283: F, t7348: F, t7362: F, t7363: F, t86032: F, t86037: F, t86095: F, t86102: F, t86106: F, t86113: F, t86116: F, t86376: F) -> (F,) {
    let t86381 = -3.0 * t3624 * t86032 * t3625 - 0.54831135561607547883e-2 * t86095 + 3.0 * t1244 * t7348 * t3493 * t1246 + 0.82246703342411321826e-2 * t86037 * t24667 * t3611 * t86102 + 0.36554090374405031922e-2 * t86106 - 0.27415567780803773942e-2 * t7283 * t7362 * t7363 * t11172 + t11616 * t2152 - 0.82246703342411321826e-2 * t86113 - 0.16449340668482264365e-1 * t24849 * t86116 * t24852 + t470 * t493 * t86376 + 6.0 * t3604 * t24841;
    (t86381,)
}
