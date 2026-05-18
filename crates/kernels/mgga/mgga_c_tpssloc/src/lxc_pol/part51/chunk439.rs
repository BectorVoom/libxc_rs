//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 439/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk439<F: Float>(t225: F, t562: F, t567: F, t214: F, t1985: F, t1878: F, t1887: F, t534: F) -> (F, F, F, F) {
    let t1987 = t562 * t225 * t567;
    let t1988 = t214 * t1987;
    let t1989 = t1985 * t1988;
    let t1992 = t1878 * t534 * t1887;
    (t1987, t1988, t1989, t1992)
}
