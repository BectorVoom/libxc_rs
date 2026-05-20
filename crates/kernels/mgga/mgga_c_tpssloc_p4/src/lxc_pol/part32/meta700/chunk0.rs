//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2193/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2193<F: Float>(t28200: F, t6883: F, t225: F, t28053: F, t6888: F, t7691: F, t90739: F, t1375: F, t1386: F, t20025: F, t2016: F, t26224: F, t26225: F, t26366: F, t3887: F, t5210: F, t5354: F, t539: F, t56422: F, t568: F, t6460: F, t6992: F, t7722: F, t81399: F, t93906: F, t97468: F) -> F {
    let t97750 = t6883 * t28200;
    let t97756 = t28053 * t225;
    let t97766 = t6888 * t90739 * t7691;
    let t97770 = F::new(2.0) * t1375 * t3887 * t6992 * t6460 - F::cast_from(0.19190897446562641759e-1_f64) * t97750 + t539 * t97468 * t568 + t93906 - F::new(2.0) * t56422 * t2016 - F::new(2.0) * t97756 * t1386 + F::new(2.0) * t5210 * t7722 * t568 - F::new(6.0) * t26224 * t26225 * t20025 - F::cast_from(0.3289868133696452873e-1_f64) * t97766 - t81399 - F::new(2.0) * t26366 * t5354;
    t97770
}
