//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 963/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk963<F: Float>(t10544: F, t2784: F, t892: F, t2841: F, t888: F, t2840: F, t287: F, t275: F, t10294: F, t891: F, t2843: F, t290: F) -> (F, F, F, F, F, F, F, F) {
    let t10636 = F::cast_from(0.55403703703703703703e-1_f64) * t10544;
    let t10650 = t2784 * t892;
    let t10655 = t888 * t2841;
    let t10660 = F::new(1.0) / t2840 / t287;
    let t10661 = t275 * t10660;
    let t10675 = F::cast_from(0.36514074074074074075e0_f64) * t10294;
    let t10676 = F::cast_from(0.93011851851851851854e0_f64) * t10544;
    let t10701 = F::new(1.0) / t2840 / t891;
    let t10702 = t275 * t10701;
    let t10704 = F::new(1.0) / t2843 / t290;
    (t10636, t10650, t10655, t10661, t10675, t10676, t10702, t10704)
}
