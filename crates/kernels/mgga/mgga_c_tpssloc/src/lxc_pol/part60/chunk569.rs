//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 569/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk569<F: Float>(t1170: F, t2123: F, t2121: F, t2127: F, t6686: F) -> (F, F, F) {
    let t7280 = t1170 * t2123;
    let t7282 = F::cast_from(0.27415567780803773942e-2_f64) * t2121 * t7280;
    let t7283 = t2127 * t6686;
    (t7280, t7282, t7283)
}
