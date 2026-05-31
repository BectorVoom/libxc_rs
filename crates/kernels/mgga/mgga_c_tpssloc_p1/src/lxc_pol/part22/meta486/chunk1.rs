//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1905/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1905<F: Float>(t13598: F, t13642: F, t17149: F, t17165: F, t17175: F, t17286: F, t17288: F, t17290: F, t21161: F, t21168: F, t21181: F, t21183: F, t21186: F, t21188: F) -> F {
    let t21237 = -F::cast_from(0.27595e0_f64) * t13642 + F::cast_from(0.49671e0_f64) * t21161 - F::cast_from(0.40256666666666666668e0_f64) * t13598 + F::cast_from(0.20128333333333333333e0_f64) * t17149 - F::cast_from(0.60385000000000000001e0_f64) * t17165 + F::cast_from(0.30192500000000000001e0_f64) * t17175 - F::cast_from(0.82785e-1_f64) * t21168 + F::cast_from(0.258925e1_f64) * t21181 + F::cast_from(0.16504875e0_f64) * t21183 - F::cast_from(0.412621875e-1_f64) * t21186 + F::cast_from(0.19419375e1_f64) * t21188 + F::cast_from(0.5519e-1_f64) * t17286 - F::cast_from(0.33114e0_f64) * t17288 + F::cast_from(0.16557e0_f64) * t17290;
    t21237
}
