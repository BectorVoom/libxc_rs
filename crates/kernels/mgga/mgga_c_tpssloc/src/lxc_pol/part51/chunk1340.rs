//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1340/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1340<F: Float>(t1799: F, t2006: F, t1307: F, t26331: F, t26446: F, t1992: F, t550: F, t6976: F, t90942: F, t32745: F, t6914: F, t1351: F, t7722: F) -> (F, F, F, F, F) {
    let t120437 = t2006 * t1799;
    let t120441 = F::new(0.9869604401089358619e-1) * t26331 * t26446 * t120437 * t1307;
    let t120445 = F::new(0.16449340668482264365e-1) * t1992 * t6976 * t90942 * t550;
    let t120446 = t6914 * t32745;
    let t120447 = F::new(0.38381794893125283518e-1) * t120446;
    let t120452 = F::new(0.16449340668482264365e-1) * t1992 * t6976 * t7722 * t1351 * t550;
    (t120437, t120441, t120445, t120447, t120452)
}
