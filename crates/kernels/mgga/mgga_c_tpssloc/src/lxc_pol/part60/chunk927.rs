//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 927/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk927<F: Float>(t27188: F, t7461: F, t28835: F, t8607: F, t128920: F, t1873: F, t7467: F, t128402: F, t33234: F, t28017: F, t7042: F, t128521: F, t2039: F, t128296: F, t33211: F, t7801: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t128924 = 4.0 * t27188 * t7461;
    let t128926 = 3.0 * t8607 * t28835;
    let t128928 = 4.0 * t128920 * t1873;
    let t128930 = 4.0 * t27188 * t7467;
    let t128932 = 2.0 * t128402 * t1873;
    let t128934 = 4.0 * t33234 * t7467;
    let t128936 = 2.0 * t7042 * t28017;
    let t128942 = 2.0 * t128521 * t2039;
    let t128953 = 4.0 * t128296 * t2039;
    let t128955 = 4.0 * t33211 * t7801;
    (t128924, t128926, t128928, t128930, t128932, t128934, t128936, t128942, t128953, t128955)
}
