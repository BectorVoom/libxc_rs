//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1341/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1341<F: Float>(t10348: F, t135: F, t973: F, t3014: F, t10263: F, t10349: F, t2960: F, t3011: F, t340: F, t343: F, t42903: F, t42906: F, t42909: F, t42911: F, t42914: F, t42916: F, t42918: F, t974: F) -> F {
    let t42925 = t973 * t135 * t10348;
    let t42927 = t3014 * t3014;
    let t42933 = -F::cast_from(0.12345679012345679012e-2_f64) * t42903 + F::cast_from(0.11111111111111111111e-2_f64) * t42906 - F::cast_from(0.11111111111111111111e-2_f64) * t42909 - F::cast_from(0.59259259259259259257e-2_f64) * t42911 + F::cast_from(0.11111111111111111111e-2_f64) * t42914 + F::cast_from(0.88888888888888888887e-2_f64) * t42916 - F::cast_from(0.32592592592592592592e-1_f64) * t42918 - F::cast_from(0.48888888888888888888e-1_f64) * t10263 * t3011 + F::cast_from(0.88888888888888888888e-2_f64) * t2960 * t10349 - F::cast_from(0.11111111111111111111e-2_f64) * t42925 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t974 * t340 * t42927 * t343;
    t42933
}
