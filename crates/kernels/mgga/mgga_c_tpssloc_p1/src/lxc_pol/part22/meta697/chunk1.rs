//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2280/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2280<F: Float>(t15578: F, t4889: F, t11789: F, t1227: F, t248: F, t5979: F, t19051: F, t3523: F, t19080: F, t3572: F, t11709: F, t18356: F) -> (F, F, F, F, F) {
    let t65637 = t4889 * t15578;
    let t65647 = t1227 * t248 * t11789 * t5979;
    let t65649 = t19051 * t3523;
    let t65651 = t19080 * t3572;
    let t65660 = t11709 * t18356;
    (t65637, t65647, t65649, t65651, t65660)
}
