//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 776/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk776<F: Float>(t1052: F, t1635: F, t1920: F, t1956: F, t388: F, t4557: F, t4660: F, t6685: F, t6687: F, t6771: F, t7554: F, t7557: F, t7562: F, t7566: F, t7569: F, t7594: F, t7600: F, t7625: F) -> F {
    let t7627 = t6685 + F::new(0.27415567780803773942e-2) * t6687 * t7554 - F::new(0.82246703342411321825e-2) * t6687 * t7557 + F::new(0.82246703342411321825e-2) * t1920 * t7562 - F::new(0.82246703342411321825e-2) * t6687 * t7566 + t7569 * t388 + t7594 * t388 - t6771 * t1635 - t4557 * t1956 - t4660 * t1956 + F::new(2.0) * t1052 * t7600 - t1052 * t7625;
    t7627
}
