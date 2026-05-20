//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2020/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2020<F: Float>(t24525: F, t9239: F, t39063: F, t7245: F, t39054: F, t50: F, t9300: F, t11588: F, t2127: F, t221: F) -> (F, F, F, F, F) {
    let t85480 = t9239 * t24525;
    let t85501 = t39063 * t7245;
    let t85536 = t39054 * t7245;
    let t85539 = t50 * t9300;
    let t85639 = t2127 * t221 * t11588;
    (t85480, t85501, t85536, t85539, t85639)
}
