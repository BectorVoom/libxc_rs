//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1173/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1173<F: Float>(t1385: F, t31558: F, t22635: F, t1992: F, t8636: F, t3887: F, t794: F, t8611: F, t6897: F, t1323: F, t8617: F, t31153: F) -> (F, F, F, F, F, F, F, F) {
    let t31559 = t31558 * t1385;
    let t31560 = t22635 * t31559;
    let t31561 = t1992 * t31560;
    let t31563 = t8636 * t1385;
    let t31564 = t3887 * t31563;
    let t31569 = t794 * t8611;
    let t31570 = t6897 * t31569;
    let t31571 = F::new(0.41123351671205660912e-2) * t31570;
    let t31573 = t1323 * t8617;
    let t31576 = F::new(0.11304371706359309439e-1) * t31153;
    (t31559, t31560, t31561, t31564, t31569, t31571, t31573, t31576)
}
