//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2679/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2679<F: Float>(t12251: F, t12267: F, t12429: F, t16155: F, t16233: F, t16244: F, t16405: F, t3783: F, t3805: F, t40188: F, t40190: F, t40206: F, t40282: F, t5245: F, t5252: F, t5301: F, t54556: F, t54557: F, t54561: F, t54567: F, t54582: F) -> F {
    let t54584 = -t54556 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t54557 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t3783 * t16155 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t54561 + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t40188 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t40190 + F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t54567 + t12267 * t5245 * t5252 / F::cast_from(512.0_f64) - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t12429 * t16405 + t16233 * t3805 * t5301 * t12251 / F::cast_from(128.0_f64) + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t40206 + t12429 * t16244 / F::cast_from(128.0_f64) + F::cast_from(119.0_f64) / F::cast_from(576.0_f64) * t40282 + F::cast_from(455.0_f64) / F::cast_from(648.0_f64) * t54582;
    t54584
}
