//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1121/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1121<F: Float>(t1720: F, t3590: F, t15425: F, t491: F, t1235: F, t4940: F, t225: F, t5053: F, t1190: F, t5052: F, t15771: F, t466: F, t11613: F, t11925: F, t11928: F, t1252: F, t1761: F, t3487: F, t3593: F, t3600: F, t3631: F, t4945: F, t498: F, t5060: F, t5089: F) -> (F,) {
    let t15808 = t1720 * t3590;
    let t15814 = t15425 * t491;
    let t15816 = t4940 * t1235;
    let t15820 = t5053 * t225;
    let t15823 = t1190 * t5052;
    let t15831 = t466 * t15771;
    let t15833 = -2.0 * t11613 * t1761 - t11925 * t1761 - t11928 * t1761 - 2.0 * t1252 * t15820 + t15808 * t498 + t15814 * t498 + 2.0 * t15816 * t498 + 2.0 * t15823 * t498 + t15831 * t498 - 2.0 * t3487 * t5089 + 4.0 * t3593 * t5060 + 2.0 * t3600 * t4945 - t3631 * t4945;
    (t15833,)
}
