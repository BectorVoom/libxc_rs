//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2196/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2196<F: Float>(t86752: F, t86801: F, t87952: F, t88001: F, t25608: F, t381: F, t25428: F, t6712: F, t13797: F, t1926: F, t221: F, t10216: F, t387: F) -> (F, F, F, F, F) {
    let t88003 = t86752 + t86801 + t87952 + t88001;
    let t88004 = t25608 * t381;
    let t88016 = t6712 * t25428;
    let t88022 = t1926 * t221 * t13797;
    let t88023 = t387 * t10216;
    (t88003, t88004, t88016, t88022, t88023)
}
