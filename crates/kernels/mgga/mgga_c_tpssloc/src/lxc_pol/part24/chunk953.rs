//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 953/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk953<F: Float>(t11628: F, t3396: F, t1164: F, t11128: F, t11133: F, t11179: F, t11182: F, t11184: F, t11187: F, t11405: F, t11409: F, t11426: F, t11429: F, t3395: F, t3400: F, t4883: F) -> (F, F, F) {
    let t11629 = t11628 * t3396;
    let t11631 = 0.35089341735807877242e1 * t1164 * t11629;
    let t11632 = -t11426 + t11429 - t11405 + t11409 + t11631 - t11128 - t11133 + t11179 + t11182 + t11184 + t11187;
    let t11634 = t3400 * t3395 * t4883;
    (t11631, t11632, t11634)
}
