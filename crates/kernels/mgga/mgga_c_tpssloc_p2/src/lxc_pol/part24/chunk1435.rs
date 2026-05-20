//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1435/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1435<F: Float>(t1873: F, t3941: F, t9416: F, t16535: F, t6534: F, t45557: F, t45560: F, t7015: F, t20173: F, t23896: F, t112: F, t23862: F) -> (F, F, F, F, F, F) {
    let t83991 = F::new(27.0) * t3941 * t1873 * t9416;
    let t83993 = F::new(81.0) * t16535 * t6534;
    let t83999 = F::new(0.135e2) * t45557 * t1873;
    let t84001 = F::new(81.0) * t45560 * t7015;
    let t84003 = F::new(81.0) * t20173 * t23896;
    let t84004 = t23862 * t112;
    (t83991, t83993, t83999, t84001, t84003, t84004)
}
