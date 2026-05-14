//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 882/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk882<F: Float>(t33310: F, t6883: F, t3886: F, t7936: F, t1385: F, t1992: F, t22635: F, t31559: F, t90566: F, t33246: F, t1985: F, t214: F, t225: F, t27051: F, t567: F, t22666: F, t33296: F) -> (F, F, F, F, F, F) {
    let t122133 = t6883 * t33310;
    let t122142 = t3886 * t7936;
    let t122145 = t1992 * t22635 * t122142 * t1385;
    let t122150 = t1992 * t90566 * t31559;
    let t122152 = t6883 * t33246;
    let t122160 = t1985 * t214 * t27051 * t225 * t567;
    let t122164 = t1985 * t22666 * t33296;
    (t122133, t122145, t122150, t122152, t122160, t122164)
}
