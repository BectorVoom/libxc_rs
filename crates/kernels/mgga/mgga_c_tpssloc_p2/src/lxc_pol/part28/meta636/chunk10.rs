//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2030/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2030<F: Float>(t91531: F, t91548: F, t12033: F, t16022: F, t26990: F, t27115: F, t3752: F, t3882: F, t568: F, t7199: F, t7214: F, t7918: F, t7937: F, t81393: F, t81395: F, t84705: F, t91505: F) -> F {
    let t93899 = F::cast_from(0.52089578783527170489e-1_f64) * t91531;
    let t93906 = F::cast_from(0.3289868133696452873e-1_f64) * t91548;
    let t93914 = -t93899 - t12033 * t7937 - F::cast_from(2.0_f64) * t3882 * t27115 - F::cast_from(0.76763589786250567036e-1_f64) * t81393 + F::cast_from(4.0_f64) * t16022 * t7199 + t93906 + t3752 * t7918 * t568 - F::cast_from(2.0_f64) * t16022 * t7214 + F::cast_from(0.76763589786250567036e-1_f64) * t81395 - F::cast_from(12.0_f64) * t91505 * t26990 - t84705;
    t93914
}
