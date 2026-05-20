//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2204/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2204<F: Float>(t13977: F, t13982: F, t13987: F, t14189: F, t23437: F, t23537: F, t4596: F, t4600: F, t4652: F, t6765: F, t82859: F, t82861: F, t82863: F, t82871: F, t82875: F, t82877: F, t83043: F, t83054: F, t83061: F) -> F {
    let t88275 = -t83061 * t4600 / F::new(768.0) + t82859 / F::new(1152.0) - t82861 / F::new(2304.0) - t82863 / F::new(324.0) - t23437 * t4652 / F::new(144.0) + F::new(5.0) / F::new(10368.0) * t82871 - t82875 / F::new(5184.0) - t82877 / F::new(1728.0) + F::new(5.0) / F::new(2592.0) * t6765 * t14189 + t83043 * t4596 / F::new(384.0) + t23537 * t13977 / F::new(384.0) + t23537 * t13982 / F::new(768.0) + t83054 * t13987 / F::new(256.0);
    t88275
}
