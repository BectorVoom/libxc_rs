//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2288/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2288<F: Float>(t100930: F, t1873: F, t20162: F, t6534: F, t26545: F, t33185: F, t12524: F, t28896: F, t3941: F, t5493: F, t2174: F, t6470: F) -> (F, F, F, F, F, F) {
    let t100932 = F::new(27.0) * t100930 * t1873;
    let t100934 = F::new(0.135e2) * t20162 * t6534;
    let t100936 = F::new(54.0) * t33185 * t26545;
    let t100938 = F::new(54.0) * t12524 * t28896;
    let t100941 = F::new(27.0) * t3941 * t6534 * t5493;
    let t103103 = t6470 * t2174;
    (t100932, t100934, t100936, t100938, t100941, t103103)
}
