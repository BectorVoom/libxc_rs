//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 934/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk934<F: Float>(t123373: F, t123981: F, t124292: F, t124383: F, t124428: F, t124472: F, t124552: F, t124584: F, t1858: F, t8811: F, t2105: F, t7945: F, t116385: F, t116387: F, t117347: F, t117349: F, t123337: F, t1404: F, t27241: F, t3: F, t32282: F, t34077: F, t5381: F, t580: F, t8812: F) -> (F, F) {
    let t124587 = t123373 + t123981 + t124292 + t124383 + t124428 + t124472 + t124552 + t124584;
    let t124591 = t8811 * t1858;
    let t124596 = t7945 * t2105;
    let t124600 = t124587 * t3 * t580 + t1404 * t34077 + t1858 * t32282 + 2.0 * t2105 * t27241 + t5381 * t8812 + t116385 + t116387 + 2.0 * t117347 + 2.0 * t117349 + t123337 + t124591 + 2.0 * t124596;
    (t124587, t124600)
}
