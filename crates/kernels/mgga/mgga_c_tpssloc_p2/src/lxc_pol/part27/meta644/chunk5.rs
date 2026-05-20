//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2203/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2203<F: Float>(t14025: F, t23537: F, t13970: F, t23541: F, t13991: F, t14107: F, t14143: F, t14147: F, t14180: F, t14184: F, t14235: F, t23419: F, t23529: F, t4585: F, t4590: F, t6765: F, t82843: F, t82851: F, t83058: F, t83065: F) -> F {
    let t88249 = t23537 * t14025 / F::new(576.0);
    let t88251 = t23541 * t13970 / F::new(1152.0);
    let t88254 = t23529 * t4585 / F::new(108.0) - F::new(5.0) / F::new(648.0) * t23529 * t4590 + F::new(5.0) / F::new(3456.0) * t6765 * t14180 + F::new(5.0) / F::new(6912.0) * t6765 * t14184 + t83065 * t14107 / F::new(1536.0) - t6765 * t14143 / F::new(576.0) - t6765 * t14147 / F::new(1152.0) + F::new(5.0) / F::new(3456.0) * t23419 * t14235 + t82843 / F::new(3456.0) - t82851 / F::new(3456.0) + t88249 - t88251 - t83058 * t13991 / F::new(256.0);
    t88254
}
