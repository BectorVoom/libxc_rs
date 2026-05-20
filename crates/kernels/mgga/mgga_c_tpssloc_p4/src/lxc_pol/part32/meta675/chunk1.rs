//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2112/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2112<F: Float>(t1210: F, t24721: F, t27691: F, t27700: F, t86261: F, t15418: F, t2121: F, t4724: F, t24720: F, t27710: F, t24722: F, t11588: F, t4729: F) -> (F, F, F, F, F, F) {
    let t95571 = F::cast_from(0.20186378047070195428e-3_f64) * t24721 * t1210 * t27691;
    let t95573 = F::cast_from(0.20186378047070195428e-3_f64) * t86261 * t27700;
    let t95587 = t2121 * t15418 * t4724 / F::new(324.0);
    let t95588 = t27710 * t24720;
    let t95590 = F::cast_from(0.16149102437656156342e-2_f64) * t95588 * t24722;
    let t95593 = t2121 * t11588 * t4729 / F::new(216.0);
    (t95571, t95573, t95587, t95588, t95590, t95593)
}
