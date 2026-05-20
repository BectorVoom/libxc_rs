//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2017/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2017<F: Float>(t1814: F, t27105: F, t81076: F, t84481: F, t90925: F, t97023: F, t97026: F, t97030: F, t97036: F, t97040: F, t97043: F, t97046: F, t97049: F, t97055: F, t97059: F, t97063: F, t97067: F, t97070: F) -> F {
    let t102614 = -t84481 + F::cast_from(0.52089578783527170489e-1_f64) * t81076 - F::cast_from(0.3289868133696452873e-1_f64) * t97023 + F::cast_from(0.16449340668482264365e-1_f64) * t97026 - F::cast_from(0.16449340668482264365e-1_f64) * t97030 - t90925 - F::cast_from(0.3289868133696452873e-1_f64) * t97036 - F::cast_from(0.3289868133696452873e-1_f64) * t97040 - F::cast_from(0.3289868133696452873e-1_f64) * t97043 + F::cast_from(0.9869604401089358619e-1_f64) * t97046 + F::new(2.0) * t1814 * t27105 - F::cast_from(0.16449340668482264365e-1_f64) * t97049 + F::cast_from(0.16449340668482264365e-1_f64) * t97055 - F::cast_from(0.9869604401089358619e-1_f64) * t97059 - F::cast_from(0.6579736267392905746e-1_f64) * t97063 - F::cast_from(0.6579736267392905746e-1_f64) * t97067 + F::cast_from(0.3289868133696452873e-1_f64) * t97070;
    t102614
}
