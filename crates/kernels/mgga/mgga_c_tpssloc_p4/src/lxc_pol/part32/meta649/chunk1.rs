//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2074/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2074<F: Float>(t22832: F, t5234: F, t1336: F, t22759: F, t5252: F, t836: F, t5293: F, t80820: F, t1831: F, t80869: F, t22783: F, t5314: F) -> (F, F, F, F, F) {
    let t91100 = t5234 * t22832;
    let t91113 = t1336 * t22759 * t836 * t5252;
    let t91114 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t91113;
    let t91120 = t80820 * t5293;
    let t91121 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t91120;
    let t91135 = t80869 * t1831;
    let t91136 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t91135;
    let t91137 = t22783 * t5314;
    (t91100, t91114, t91121, t91136, t91137)
}
