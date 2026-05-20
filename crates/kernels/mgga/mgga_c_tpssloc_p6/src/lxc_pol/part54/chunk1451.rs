//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1451/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1451<F: Float>(t31817: F, t33185: F, t1873: F, t94127: F, t120849: F, t8657: F, t75795: F, t100993: F, t7769: F, t24465: F, t26542: F, t26545: F) -> (F, F, F, F, F, F, F) {
    let t122788 = F::new(27.0) * t33185 * t31817;
    let t122790 = F::new(0.135e2) * t94127 * t1873;
    let t122794 = F::new(27.0) * t120849 * t8657;
    let t122800 = F::new(27.0) * t75795 * t8657;
    let t122804 = F::new(27.0) * t100993 * t7769;
    let t122806 = F::new(27.0) * t24465 * t26542;
    let t122808 = F::new(27.0) * t24465 * t26545;
    (t122788, t122790, t122794, t122800, t122804, t122806, t122808)
}
