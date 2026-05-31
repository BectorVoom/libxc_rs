//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 631/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk631<F: Float>(t1425: F, t2476: F, t865: F, t2481: F, t1415: F, t2487: F, t849: F, t2455: F, t2491: F, t3746: F, t3751: F, t3756: F, t3760: F) -> (F, F, F, F, F, F) {
    let t3769 = F::cast_from(1.0_f64) * t2476 * t1425;
    let t3770 = t1425 * t865;
    let t3772 = F::cast_from(2.0_f64) * t2481 * t3770;
    let t3773 = t2487 * t1415;
    let t3774 = t3773 * t849;
    let t3781 = t2491 + t2455 / F::cast_from(9.0_f64) + t3746 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3751 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t3756 - t3760 / F::cast_from(3.0_f64);
    (t3769, t3770, t3772, t3773, t3774, t3781)
}
