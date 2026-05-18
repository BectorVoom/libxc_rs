//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 992/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk992<F: Float>(t1873: F, t28951: F, t3941: F, t7467: F, t7801: F, t2098: F, t5456: F, t28017: F, t7230: F, t55388: F, t8657: F, t33211: F, t7802: F) -> (F, F, F, F, F, F) {
    let t127698 = F::new(27.0) * t3941 * t28951 * t1873;
    let t127701 = F::new(54.0) * t3941 * t7801 * t7467;
    let t127704 = t2098 * t5456;
    let t127706 = F::new(27.0) * t127704 * t1873;
    let t127708 = F::new(0.135e2) * t7230 * t28017;
    let t127714 = F::new(27.0) * t55388 * t8657;
    let t127720 = F::new(4.0) * t33211 * t7802;
    (t127698, t127701, t127706, t127708, t127714, t127720)
}
