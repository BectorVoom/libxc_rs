//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 929/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk929<F: Float>(t6883: F, t7697: F, t22751: F, t7733: F, t22893: F, t7732: F, t22892: F, t1834: F, t552: F, t6914: F, t7737: F, t1799: F, t562: F, t22705: F, t7736: F, t22704: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26361 = t6883 * t7697;
    let t26381 = t22751 * t7733;
    let t26392 = t22893 * t7732;
    let t26393 = t22892 * t26392;
    let t26395 = t552 * t1834;
    let t26406 = t6914 * t7737;
    let t26421 = t562 * t1799;
    let t26426 = t22705 * t7736;
    let t26427 = t22704 * t26426;
    (t26361, t26381, t26392, t26393, t26395, t26406, t26421, t26426, t26427)
}
