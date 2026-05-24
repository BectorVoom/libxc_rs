//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 936/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk936<F: Float>(t1763: F, t352: F, t1971: F, t3351: F, t4617: F, t45343: F, t674: F, t2007: F, t321: F, t9888: F, t262: F, t36629: F) -> (F, F, F, F, F, F, F) {
    let t45556 = t1763 * t352;
    let t45559 = t3351 * t1971 * t4617 * t45556;
    let t45561 = t45343 * t674;
    let t45562 = t45561 * t2007;
    let t45568 = t9888 * t321;
    let t45569 = t262 * t45568;
    let t45570 = t36629 * t45569;
    (t45556, t45559, t45561, t45562, t45568, t45569, t45570)
}
