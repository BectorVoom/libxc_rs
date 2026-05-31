//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2298/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2298<F: Float>(t6238: F, t7284: F, t24574: F, t29546: F, t103314: F, t1090: F, t11605: F, t1238: F, t1251: F, t1761: F, t24589: F, t24601: F, t24893: F, t27382: F, t27406: F, t27742: F, t27784: F, t27792: F, t27821: F, t27826: F, t27830: F, t29794: F, t3598: F, t4930: F, t4945: F, t5059: F, t5060: F, t5089: F, t6244: F, t7283: F, t7287: F, t8087: F, t94395: F, t94648: F, t94656: F) -> F {
    let t103391 = t7284 * t6238;
    let t103413 = t24574 * t29546;
    let t103415 = -F::cast_from(2.0_f64) * t27830 * t5089 + F::cast_from(2.0_f64) * t24893 * t6244 - F::cast_from(12.0_f64) * t27784 * t11605 * t8087 * t5059 + t94648 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t4930 * t27382 - F::cast_from(2.0_f64) * t94656 * t1761 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t103391 * t7287 + F::cast_from(2.0_f64) * t1238 * t3598 * t29794 * t1251 + F::cast_from(4.0_f64) * t27792 * t5060 - F::cast_from(2.0_f64) * t4945 * t27742 + F::cast_from(0.43864908449286038306e-1_f64) * t27406 * t27826 + F::cast_from(0.27415567780803773942e-2_f64) * t24589 * t24601 * t103314 * t1090 - F::cast_from(0.14621636149762012769e-1_f64) * t94395 * t27821 - F::cast_from(2.0_f64) * t27792 * t5089 - F::cast_from(0.27415567780803773942e-2_f64) * t103413;
    t103415
}
