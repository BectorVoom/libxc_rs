//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1983/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1983<F: Float>(t2047: F, t5611: F, t5584: F, t101698: F, t13176: F, t16935: F, t2617: F, t26608: F, t26656: F, t29010: F, t4166: F, t4182: F, t4234: F, t4281: F, t4291: F, t7837: F, t829: F, t85003: F, t87635: F, t87653: F, t87666: F, t92760: F, t92768: F, t92795: F, t98575: F) -> (F, F) {
    let t101708 = t2047 * t5611;
    let t101715 = t2047 * t5584;
    let t101734 = -F::cast_from(0.6579736267392905746e-1_f64) * t98575 - t92760 + F::new(2.0) * t4281 * t101708 * t4182 + t92768 - F::new(2.0) * t4291 * t26656 * t4234 - t4291 * t101715 * t829 - F::cast_from(0.5117572652416704469e0_f64) * t87635 - t4291 * t101708 * t829 - F::cast_from(0.3289868133696452873e-1_f64) * t87653 + t85003 - t2617 * t29010 + F::new(4.0) * t4281 * t26656 * t16935 - F::new(2.0) * t13176 * t7837 - F::new(2.0) * t4166 * t26608 + F::new(4.0) * t4281 * t101698 * t4182 - F::cast_from(0.25587863262083522345e0_f64) * t87666 + t92795;
    (t101715, t101734)
}
