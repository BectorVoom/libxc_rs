//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 772/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk772<F: Float>(t483: F, t493: F, t470: F, t2134: F, t488: F, t8875: F) -> (F, F, F) {
    let t8878 = t493 * t483;
    let t8879 = t470 * t8878;
    let t8882 = 0.40372756094140390856e-3 * t2134 * t8875 + t8879 * t488 / 1536.0;
    (t8878, t8879, t8882)
}
