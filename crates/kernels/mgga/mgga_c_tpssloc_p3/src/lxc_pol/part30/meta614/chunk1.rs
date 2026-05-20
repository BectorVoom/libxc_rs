//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2013/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2013<F: Float>(t10948: F, t23540: F, t10472: F, t10478: F, t6753: F, t10375: F, t1942: F, t23488: F, t23509: F, t23508: F, t6721: F, t6741: F) -> (F, F, F, F, F) {
    let t83061 = t10948 * t23540;
    let t83065 = t10472 * t6753 * t10478;
    let t83080 = t1942 * t10375 / F::new(5184.0);
    let t83117 = t23509 * t23488;
    let t83120 = t6721 * t23508;
    let t83121 = t83120 * t6741;
    (t83061, t83065, t83080, t83117, t83121)
}
