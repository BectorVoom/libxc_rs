//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2271/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2271<F: Float>(t12725: F, t12734: F, t1442: F, t1458: F, t2314: F, t24924: F, t27879: F, t4026: F, t652: F, t7271: F, t7408: F, t7989: F, t90022: F, t90026: F, t90029: F, t90034: F, t90036: F, t90038: F, t90040: F, t90051: F, t90059: F, t90062: F, t90064: F, t90068: F, t90418: F) -> F {
    let t94236 = -F::new(2.0) * t1458 * t24924 * t652 - F::new(4.0) * t12725 * t7271 - F::new(4.0) * t12734 * t7989 - t1442 * t24924 - F::new(4.0) * t2314 * t27879 - F::new(2.0) * t4026 * t7408 + t90022 + t90026 - t90029 + t90034 - t90036 - t90038 + t90040 - t90051 - t90059 + t90062 + t90064 + t90068 + t90418;
    t94236
}
