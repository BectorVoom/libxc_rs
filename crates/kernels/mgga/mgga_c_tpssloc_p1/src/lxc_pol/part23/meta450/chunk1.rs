//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1298/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1298<F: Float>(t57960: F, t46208: F, t57992: F, t1462: F, t67181: F, t16625: F, t20947: F, t2522: F, t39463: F, t39468: F, t39472: F, t39476: F, t40714: F, t40716: F, t40721: F, t40732: F, t4310: F, t4314: F, t5544: F) -> (F, F, F, F, F) {
    let t75864 = F::cast_from(48.0_f64) * t57960;
    let t75865 = F::cast_from(0.4101607543286562663e4_f64) * t46208;
    let t75872 = F::cast_from(24.0_f64) * t57992;
    let t75874 = F::cast_from(16.0_f64) * t67181 * t1462;
    let t75875 = -F::cast_from(18.0_f64) * t16625 * t2522 * t5544 + F::cast_from(72.0_f64) * t20947 * t4310 * t4314 + t39463 - t39468 - t39472 - t39476 - t40714 + t40716 - t40721 - t40732 + t75864 - t75865 + t75872 + t75874;
    (t75864, t75865, t75872, t75874, t75875)
}
