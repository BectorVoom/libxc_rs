//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1047/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1047<F: Float>(t128521: F, t2039: F, t128296: F, t33211: F, t7801: F, t28017: F, t88: F, t33596: F, t28951: F, t8601: F, t102386: F, t1873: F) -> (F, F, F, F, F, F, F) {
    let t128942 = F::new(2.0) * t128521 * t2039;
    let t128953 = F::new(4.0) * t128296 * t2039;
    let t128955 = F::new(4.0) * t33211 * t7801;
    let t128956 = t88 * t28017;
    let t128958 = F::new(2.0) * t128956 * t2039;
    let t128960 = F::new(4.0) * t33596 * t7801;
    let t128962 = F::new(2.0) * t8601 * t28951;
    let t128968 = F::new(2.0) * t102386 * t1873;
    (t128942, t128953, t128955, t128958, t128960, t128962, t128968)
}
