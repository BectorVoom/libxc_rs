//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2015/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2015<F: Float>(t90898: F, t90900: F, t1336: F, t16206: F, t27097: F, t27098: F, t3777: F, t3851: F, t7208: F, t81037: F, t81039: F, t81041: F, t81043: F, t81047: F, t81050: F, t81061: F, t81066: F, t90883: F, t90887: F, t90892: F, t90895: F) -> F {
    let t93562 = F::cast_from(0.3289868133696452873e-1_f64) * t90898;
    let t93563 = F::cast_from(0.52089578783527170489e-1_f64) * t90900;
    let t93567 = -F::cast_from(0.38381794893125283518e-1_f64) * t81037 + F::cast_from(0.25587863262083522346e0_f64) * t81039 + F::cast_from(0.38381794893125283518e-1_f64) * t81041 - t1336 * t27097 * t3851 - F::cast_from(0.23029076935875170111e0_f64) * t81043 - F::cast_from(0.10417915756705434098e0_f64) * t81047 + F::cast_from(0.16449340668482264365e-1_f64) * t81050 - t1336 * t7208 * t16206 - F::cast_from(0.3289868133696452873e-1_f64) * t90883 - F::cast_from(0.16449340668482264365e-1_f64) * t90887 - F::cast_from(0.25587863262083522346e0_f64) * t81061 - F::cast_from(0.6579736267392905746e-1_f64) * t90892 + F::cast_from(0.6579736267392905746e-1_f64) * t90895 - t93562 + t93563 - F::new(2.0) * t3777 * t27098 + F::cast_from(0.3289868133696452873e-1_f64) * t81066;
    t93567
}
