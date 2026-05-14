//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1207/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1207<F: Float>(t100137: F, t100189: F, t100193: F, t100231: F, t106028: F, t1610: F, t21510: F, t21643: F, t23327: F, t23601: F, t23633: F, t23677: F, t23678: F, t25470: F, t25510: F, t25511: F, t28617: F, t28634: F, t28642: F, t3200: F, t3201: F, t4669: F, t5866: F, t5903: F, t7603: F, t7619: F, t7622: F) -> (F,) {
    let t106113 = 0.16449340668482264365e-1 * t23327 * t25470 * t28617 + 3.0 * t1610 * t28634 + 3.0 * t5903 * t7622 + 0.82246703342411321826e-2 * t23633 * t100231 * t106028 - 0.16449340668482264365e-1 * t23327 * t25510 * t25511 * t21510 + 0.49348022005446793095e-1 * t23601 * t23677 * t21643 * t23678 - 0.82246703342411321826e-2 * t23327 * t100137 * t7603 - 3.0 * t3200 * t7619 * t3201 * t5866 + 0.36554090374405031922e-2 * t100189 + 3.0 * t4669 * t28642 - 0.16449340668482264365e-1 * t100193;
    (t106113,)
}
