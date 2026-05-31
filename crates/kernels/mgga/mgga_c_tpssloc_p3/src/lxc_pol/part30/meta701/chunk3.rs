//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2268/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2268<F: Float>(t193: F, t200: F, t7540: F, t1408: F, t16557: F, t1877: F, t1915: F, t22959: F, t23295: F, t25: F, t25013: F, t25015: F, t25021: F, t2522: F, t25354: F, t25366: F, t25372: F, t25385: F, t7541: F, t86736: F, t98091: F, t98094: F, t98103: F, t98112: F, t99043: F, t99049: F, t99055: F, t99056: F, t99060: F) -> (F, F) {
    let t99064 = t193 * t200 * t7540;
    let t99067 = t1877 * t23295 * t98091 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t98094 - F::cast_from(3.0_f64) * t86736 * t25021 - F::cast_from(3.0_f64) * t86736 * t25366 + t25372 * t98103 + F::cast_from(3.0_f64) * t2522 * t7541 * t25385 + t1877 * t1915 * t16557 / F::cast_from(2.0_f64) + F::cast_from(6.0_f64) * t22959 * t98112 + t1877 * t99043 * t25 / F::cast_from(2.0_f64) + t1877 * t25354 * t1408 - F::cast_from(3.0_f64) * t22959 * t99049 + t99055 + F::cast_from(3.0_f64) * t25013 * t99056 + F::cast_from(6.0_f64) * t25013 * t99060 + F::cast_from(6.0_f64) * t99064 * t25015;
    (t99064, t99067)
}
