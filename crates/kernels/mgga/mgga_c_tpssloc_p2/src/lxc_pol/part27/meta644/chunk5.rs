//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2203/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2203<F: Float>(t14025: F, t23537: F, t13970: F, t23541: F, t13991: F, t14107: F, t14143: F, t14147: F, t14180: F, t14184: F, t14235: F, t23419: F, t23529: F, t4585: F, t4590: F, t6765: F, t82843: F, t82851: F, t83058: F, t83065: F) -> F {
    let t88249 = t23537 * t14025 / F::cast_from(576.0_f64);
    let t88251 = t23541 * t13970 / F::cast_from(1152.0_f64);
    let t88254 = t23529 * t4585 / F::cast_from(108.0_f64) - F::cast_from(5.0_f64) / F::cast_from(648.0_f64) * t23529 * t4590 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t6765 * t14180 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t6765 * t14184 + t83065 * t14107 / F::cast_from(1536.0_f64) - t6765 * t14143 / F::cast_from(576.0_f64) - t6765 * t14147 / F::cast_from(1152.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t23419 * t14235 + t82843 / F::cast_from(3456.0_f64) - t82851 / F::cast_from(3456.0_f64) + t88249 - t88251 - t83058 * t13991 / F::cast_from(256.0_f64);
    t88254
}
