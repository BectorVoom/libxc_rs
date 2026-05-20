//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1726/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1726<F: Float>(t3911: F, t6906: F, t6889: F, t1985: F, t1372: F, t214: F) -> (F, F, F, F) {
    let t22662 = t6906 * t3911;
    let t22663 = t6889 * t22662;
    let t22664 = t1985 * t22663;
    let t22666 = t214 * t1372;
    (t22662, t22663, t22664, t22666)
}
