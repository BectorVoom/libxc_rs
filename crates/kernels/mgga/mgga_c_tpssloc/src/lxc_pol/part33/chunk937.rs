//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 937/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk937<F: Float>(t1380: F, t20568: F, t1825: F, t19660: F, t5348: F, t6420: F, t20473: F, t5335: F, t20554: F, t6415: F, t19657: F, t16428: F, t6388: F) -> (F, F, F, F, F, F, F, F) {
    let t20630 = t1380 * t20568;
    let t20632 = t19660 * t1825;
    let t20635 = t5348 * t6420;
    let t20638 = t5335 * t20473;
    let t20643 = t1380 * t20554;
    let t20645 = t5348 * t6415;
    let t20648 = t19657 * t1825;
    let t20651 = t16428 * t6388;
    (t20630, t20632, t20635, t20638, t20643, t20645, t20648, t20651)
}
