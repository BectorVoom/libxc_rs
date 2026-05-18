//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 682/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk682<F: Float>(t225: F, t2711: F, t2594: F, t120: F, t2678: F, t2631: F, t2592: F, t252: F, t856: F, t68: F, t261: F, t2751: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9590 = t2711 * t225;
    let t9593 = t2594 * t225;
    let t9621 = t120 * t2678;
    let t9626 = t120 * t2631;
    let t10049 = t2592 * t225;
    let t10097 = t252 * t2678;
    let t10108 = t856 * t856;
    let t10109 = F::new(1.0) / t10108;
    let t10110 = t68 * t10109;
    let t10143 = F::new(1.0) / t2751 / t261;
    (t9590, t9593, t9621, t9626, t10049, t10097, t10108, t10109, t10110, t10143)
}
