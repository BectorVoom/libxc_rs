//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 850/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk850<F: Float>(t154: F, t9569: F, t222: F, t252: F, t9971: F, t856: F, t68: F, t261: F, t2751: F) -> (F, F, F, F, F, F, F) {
    let t10027 = t9569 * t154;
    let t10029 = F::new(455.0) / F::new(1296.0) * t10027 * t222;
    let t10080 = t9971 * t252;
    let t10108 = t856 * t856;
    let t10109 = F::new(1.0) / t10108;
    let t10110 = t68 * t10109;
    let t10143 = F::new(1.0) / t2751 / t261;
    (t10027, t10029, t10080, t10108, t10109, t10110, t10143)
}
