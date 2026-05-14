//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 642/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk642<F: Float>(t1015: F, t1114: F, t3068: F, t2856: F, t2859: F, t2866: F, t2908: F, t2916: F, t3006: F, t3008: F, t3011: F, t3015: F, t3019: F, t3023: F) -> (F, F, F) {
    let t3069 = t1114 * t1015;
    let t3070 = t3068 * t3069;
    let t3073 = -t2856 + t2859 - t2866 + t2908 + t2916 + t3006 + t3008 - t3011 + t3015 - t3019 - t3023;
    (t3069, t3070, t3073)
}
