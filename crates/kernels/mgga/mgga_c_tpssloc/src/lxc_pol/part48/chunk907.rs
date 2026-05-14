//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 907/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk907<F: Float>(t117553: F, t117567: F, t117590: F, t117604: F, t117622: F, t117634: F, t117648: F, t117659: F, t114456: F, t114472: F, t114513: F, t114515: F, t114517: F, t114520: F, t114525: F, t114527: F, t114529: F, t114531: F, t115978: F, t115980: F, t115983: F, t2039: F, t2363: F, t32406: F, t8508: F, t85423: F, t96316: F) -> (F, F) {
    let t117662 = t117553 + t117567 + t117590 + t117604 + t117622 + t117634 + t117648 + t117659;
    let t117671 = t114513 + t114515 + t114517 + t114520 + t114456 + t8508 + t114525 + t114527 + t114529 + t114531 + 0.135e2 * t85423 * t2039 + t115978 + t115980 + t114472 + t115983 + 27.0 * t96316 * t2039 + 0.135e2 * t32406 * t2363;
    (t117662, t117671)
}
