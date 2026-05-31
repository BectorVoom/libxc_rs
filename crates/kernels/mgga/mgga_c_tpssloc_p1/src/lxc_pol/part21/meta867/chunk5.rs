//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3169/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3169<F: Float>(t11546: F, t1174: F, t1232: F, t14744: F, t14753: F, t15569: F, t15710: F, t15764: F, t1735: F, t1743: F, t18395: F, t3447: F, t3566: F, t3577: F, t3578: F, t45119: F, t488: F, t52696: F, t52995: F, t53187: F, t55716: F, t6164: F, t63372: F, t65567: F, t65581: F, t65598: F, t65600: F, t65605: F, t65607: F) -> F {
    let t65610 = t65567 / F::cast_from(54.0_f64) - t15764 * t1743 * t488 / F::cast_from(288.0_f64) + F::cast_from(19.0_f64) / F::cast_from(1728.0_f64) * t3566 * t6164 * t488 - t3447 * t53187 * t55716 / F::cast_from(12.0_f64) + t3447 * t52995 * t55716 / F::cast_from(9.0_f64) - t65581 / F::cast_from(13824.0_f64) - t45119 * t3578 * t52696 * t18395 / F::cast_from(2304.0_f64) - t3577 * t3578 * t1735 * t14753 / F::cast_from(1152.0_f64) - t3577 * t3578 * t1735 * t14744 / F::cast_from(384.0_f64) + t15569 * t15710 / F::cast_from(108.0_f64) + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t65598 + t65600 / F::cast_from(1296.0_f64) - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1174 * t11546 * t63372 - t65605 / F::cast_from(6912.0_f64) - t65607 * t1232 / F::cast_from(2304.0_f64);
    t65610
}
