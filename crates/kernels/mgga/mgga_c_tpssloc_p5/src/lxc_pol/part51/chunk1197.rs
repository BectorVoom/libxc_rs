//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1197/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1197<F: Float>(t31528: F, t31729: F, t31751: F, t31779: F, t3: F, t112: F, t8646: F, t1873: F, t24462: F, t24465: F, t7015: F, t6534: F, t7230: F) -> (F, F, F, F, F, F) {
    let t31781 = t31528 + t31729 + t31751 + t31779;
    let t31782 = t3 * t31781;
    let t31795 = t8646 * t112;
    let t31799 = F::cast_from(0.135e2_f64) * t24462 * t1873;
    let t31801 = F::cast_from(27.0_f64) * t24465 * t7015;
    let t31803 = F::cast_from(0.135e2_f64) * t7230 * t6534;
    (t31781, t31782, t31795, t31799, t31801, t31803)
}
