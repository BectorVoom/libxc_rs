//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 975/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk975<F: Float>(t25980: F, t652: F, t22591: F, t7687: F, t1983: F, t1307: F, t1845: F, t8643: F, t22574: F, t15868: F, t2019: F, t1774: F, t6534: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25982 = F::new(2.0) * t652 * t25980;
    let t25985 = t22591 * t7687;
    let t25987 = F::new(3.0) * t1983 * t25985;
    let t25988 = t1845 * t1307;
    let t25989 = t8643 * t25988;
    let t25991 = F::new(3.0) * t22574 * t25989;
    let t25992 = t2019 * t15868;
    let t25993 = t1983 * t25992;
    let t25994 = t1774 * t6534;
    (t25982, t25985, t25987, t25988, t25989, t25991, t25992, t25993, t25994)
}
