//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2937/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2937<F: Float>(t43002: F, t60274: F, t60277: F, t60282: F, t60296: F, t60308: F, t60310: F, t60312: F, t60315: F, t60318: F, t60321: F, t60324: F, t60327: F) -> F {
    let t61163 = -t43002 - F::new(2.0) / F::new(27.0) * t60274 - F::new(2.0) / F::new(3.0) * t60277 - t60282 / F::new(3.0) - t60296 / F::new(3.0) + F::new(4.0) / F::new(9.0) * t60308 - F::new(4.0) / F::new(27.0) * t60310 - F::new(8.0) / F::new(81.0) * t60312 - t60315 / F::new(3.0) - F::new(8.0) / F::new(9.0) * t60318 + t60321 / F::new(9.0) + F::new(2.0) / F::new(27.0) * t60324 + F::new(14.0) / F::new(81.0) * t60327;
    t61163
}
