//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2054/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2054<F: Float>(t87729: F, t25325: F, t6547: F, t1911: F, t40889: F, t23185: F, t25045: F, t82074: F, t225: F, t25161: F, t6562: F, t6572: F, t86893: F) -> (F, F, F, F, F, F) {
    let t87730 = F::cast_from(0.82246703342411321824e-2_f64) * t87729;
    let t87733 = t6547 * t25325;
    let t87734 = F::cast_from(0.38381794893125283518e-1_f64) * t87733;
    let t87748 = t40889 * t1911;
    let t87753 = t23185 * t82074 * t25045;
    let t87754 = F::cast_from(0.16449340668482264365e-1_f64) * t87753;
    let t87758 = t25161 * t225;
    let t87776 = t6562 * t86893 * t6572;
    (t87730, t87734, t87748, t87754, t87758, t87776)
}
