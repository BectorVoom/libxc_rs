//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 888/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk888<F: Float>(t122448: F, t1307: F, t26331: F, t26446: F, t1992: F, t550: F, t6976: F, t93501: F, t22704: F, t22705: F, t33280: F, t33281: F, t6914: F, t1351: F, t7918: F, t1985: F, t1998: F, t214: F, t27051: F) -> (F, F, F, F, F, F) {
    let t122451 = t26331 * t26446 * t122448 * t1307;
    let t122457 = t1992 * t6976 * t93501 * t550;
    let t122460 = t22704 * t22705 * t33280;
    let t122462 = t6914 * t33281;
    let t122467 = t1992 * t6976 * t7918 * t1351 * t550;
    let t122483 = t1985 * t214 * t1998 * t27051;
    (t122451, t122457, t122460, t122462, t122467, t122483)
}
