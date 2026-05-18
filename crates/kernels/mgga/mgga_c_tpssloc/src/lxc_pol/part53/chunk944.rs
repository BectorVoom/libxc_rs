//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 944/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk944<F: Float>(t112778: F, t112803: F, t112818: F, t112820: F, t112846: F, t31386: F, t6579: F, t23012: F, t8538: F, t31339: F, t81591: F, t2047: F, t213: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114714 = F::new(0.5383034145885385447e-3) * t112778;
    let t114720 = F::new(7.0) / F::new(576.0) * t112803;
    let t114724 = F::new(0.32298204875312312682e-2) * t112818;
    let t114725 = F::new(7.0) / F::new(144.0) * t112820;
    let t114736 = F::new(7.0) / F::new(576.0) * t112846;
    let t114752 = t6579 * t31386;
    let t114759 = t23012 * t8538;
    let t114762 = t81591 * t31339;
    let t114770 = t213 * t2047 * t225;
    (t114714, t114720, t114724, t114725, t114736, t114752, t114759, t114762, t114770)
}
