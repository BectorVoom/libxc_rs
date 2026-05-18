//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1108/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1108<F: Float>(t1635: F, t30912: F, t30915: F, t32987: F, t32993: F, t32998: F, t33001: F, t33005: F, t33007: F, t388: F, t4557: F, t4660: F, t6687: F, t6771: F, t7600: F, t7625: F, t8397: F, t8407: F) -> F {
    let t33012 = F::new(2.0) * t4557 * t8397 + F::new(0.54831135561607547883e-2) * t6687 * t32987 + F::new(4.0) * t6771 * t7600 - F::new(0.16449340668482264365e-1) * t6687 * t32993 + F::new(2.0) * t4660 * t8397 - F::new(0.16449340668482264365e-1) * t6687 * t32998 - F::new(0.16449340668482264365e-1) * t6687 * t33001 - t4660 * t8407 + t33005 * t388 + t33007 * t388 - t30915 * t1635 - F::new(2.0) * t6771 * t7625 - t30912;
    t33012
}
