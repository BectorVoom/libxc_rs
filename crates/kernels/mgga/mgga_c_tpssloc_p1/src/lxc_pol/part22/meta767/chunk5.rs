//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2597/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2597<F: Float>(t1730: F, t19032: F, t1017: F, t1207: F, t1210: F, t22173: F, t372: F, t471: F, t479: F, t15507: F, t19095: F, t1218: F, t1232: F, t65660: F, t65662: F, t65664: F, t65668: F, t65670: F, t65672: F, t65674: F, t65676: F, t65681: F) -> F {
    let t72384 = t1730 * t19032;
    let t72389 = t1207 * t1210 * t22173 * t1017;
    let t72398 = t471 * t479 * t22173 * t372;
    let t72403 = t15507 * t19095;
    let t72405 = t65660 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t65662 - F::cast_from(19.0_f64) / F::cast_from(2592.0_f64) * t65664 - F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t72384 * t1232 - F::cast_from(209.0_f64) / F::cast_from(2592.0_f64) * t72389 * t1218 + t65668 / F::cast_from(216.0_f64) + F::cast_from(19.0_f64) / F::cast_from(864.0_f64) * t65670 - F::cast_from(19.0_f64) / F::cast_from(1296.0_f64) * t65672 - t65674 / F::cast_from(1536.0_f64) + F::cast_from(209.0_f64) / F::cast_from(3888.0_f64) * t72398 * t1232 - t65676 / F::cast_from(1152.0_f64) + t65681 / F::cast_from(1536.0_f64) + t72403 / F::cast_from(288.0_f64);
    t72405
}
