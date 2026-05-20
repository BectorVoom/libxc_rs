//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 882/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk882<F: Float>(t1378: F, t5353: F, t1375: F, t1386: F, t1843: F, t3758: F, t3882: F, t5211: F, t5213: F, t5215: F, t5217: F, t5319: F, t5321: F, t5326: F, t568: F) -> (F, F) {
    let t5354 = t1378 * t5353;
    let t5356 = F::new(2.0) * t1375 * t5326 - t1375 * t5354 - t1386 * t5215 - t1386 * t5321 - t1843 * t3758 - t1843 * t3882 + t5211 * t568 + t5213 * t568 + t5217 * t568 + t5319 * t568;
    (t5354, t5356)
}
