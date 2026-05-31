//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2458/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2458<F: Float>(t10956: F, t1611: F, t10517: F, t4630: F, t10459: F, t4644: F, t4608: F, t698: F, t973: F, t10398: F, t1041: F, t10419: F, t1044: F, t1046: F, t13995: F, t14085: F, t14147: F, t14187: F, t14189: F, t248: F, t3057: F, t3117: F, t43301: F, t4582: F, t4588: F, t45997: F, t47734: F, t48554: F) -> F {
    let t50334 = t1611 * t10956;
    let t50337 = t10517 * t4630;
    let t50343 = t4644 * t10459;
    let t50361 = t973 * t698 * t4608;
    let t50362 = t50361 / F::cast_from(432.0_f64);
    let t50365 = t13995 * t10398 / F::cast_from(1536.0_f64) + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t50334 * t1046 + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t50337 + t1041 * t248 * t1044 * t47734 / F::cast_from(4608.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t50343 + t14085 * t3057 / F::cast_from(1536.0_f64) - t3117 * t14147 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t3117 * t14189 + F::cast_from(5.0_f64) / F::cast_from(4608.0_f64) * t1041 * t4582 * t4588 * t45997 + F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t1041 * t4582 * t14187 * t48554 + t43301 / F::cast_from(1536.0_f64) - t50362 - t13995 * t10419 / F::cast_from(768.0_f64);
    t50365
}
