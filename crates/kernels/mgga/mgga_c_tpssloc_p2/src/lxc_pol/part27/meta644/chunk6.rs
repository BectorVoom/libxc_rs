//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2204/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2204<F: Float>(t13977: F, t13982: F, t13987: F, t14189: F, t23437: F, t23537: F, t4596: F, t4600: F, t4652: F, t6765: F, t82859: F, t82861: F, t82863: F, t82871: F, t82875: F, t82877: F, t83043: F, t83054: F, t83061: F) -> F {
    let t88275 = -t83061 * t4600 / F::cast_from(768.0_f64) + t82859 / F::cast_from(1152.0_f64) - t82861 / F::cast_from(2304.0_f64) - t82863 / F::cast_from(324.0_f64) - t23437 * t4652 / F::cast_from(144.0_f64) + F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t82871 - t82875 / F::cast_from(5184.0_f64) - t82877 / F::cast_from(1728.0_f64) + F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t6765 * t14189 + t83043 * t4596 / F::cast_from(384.0_f64) + t23537 * t13977 / F::cast_from(384.0_f64) + t23537 * t13982 / F::cast_from(768.0_f64) + t83054 * t13987 / F::cast_from(256.0_f64);
    t88275
}
