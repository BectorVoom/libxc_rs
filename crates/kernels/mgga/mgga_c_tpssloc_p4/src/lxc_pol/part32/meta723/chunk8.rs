//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2314/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2314<F: Float>(t24574: F, t29790: F, t29763: F, t8067: F, t94490: F, t11914: F, t1201: F, t1244: F, t1246: F, t18572: F, t18940: F, t19153: F, t2144: F, t2152: F, t27406: F, t27460: F, t27466: F, t27474: F, t27478: F, t29708: F, t29773: F, t4733: F, t5011: F, t5064: F, t7283: F, t7362: F, t8054: F, t95726: F) -> F {
    let t103950 = t24574 * t29790;
    let t103954 = t24574 * t29763;
    let t103959 = t94490 * t8067;
    let t103978 = -F::cast_from(0.54831135561607547883e-2_f64) * t103950 + t11914 * t29708 * t19153 - F::cast_from(0.18277045187202515961e-2_f64) * t103954 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t27466 + t18572 * t2152 + F::cast_from(0.48738787165873375897e-2_f64) * t103959 - F::cast_from(0.54831135561607547884e-2_f64) * t7283 * t7362 * t27460 * t4733 - F::cast_from(0.36554090374405031923e-2_f64) * t95726 + F::cast_from(2.0_f64) * t1244 * t8054 * t5011 * t1246 + t1201 * t29773 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t27474 + F::cast_from(2.0_f64) * t5064 * t27478 + t1244 * t2144 * t18940 * t1246;
    t103978
}
