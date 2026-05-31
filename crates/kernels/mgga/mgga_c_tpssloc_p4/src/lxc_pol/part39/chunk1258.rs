//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1258/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1258<F: Float>(t28: F, t12000: F, t1649: F, t2: F, t3711: F, t1302: F, t15956: F, t16: F, t3231: F, t3673: F, t5178: F, t5181: F, t584: F, zeta_threshold: F) -> F {
    let t29 = t28 <= zeta_threshold;
    let t16003 = t12000 * t1649;
    let t16006 = t3711 * t2;
    let t16016 = piecewise3::<F>(t29, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t16003 * t3673 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t16006 * t15956 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t5178 * t3231 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1302 * t584 + F::cast_from(4.0_f64) * t5181 * t16);
    t16016
}
