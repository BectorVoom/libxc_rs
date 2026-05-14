//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 674/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk674<F: Float>(t1153: F, t198: F, t2856: F, t2859: F, t2866: F, t2908: F, t2916: F, t3006: F, t3008: F, t3011: F, t3015: F, t3019: F, t3023: F, t3147: F, t3151: F, t3154: F, t330: F) -> (F,) {
    let t3157 = t1153 * t198 * t3147 * t330 - t198 * t3151 * t3154 * t330 - t2856 + t2859 - t2866 + t2908 + t2916 + t3006 + t3008 - t3011 + t3015 - t3019 - t3023;
    (t3157,)
}
