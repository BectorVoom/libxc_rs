//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1217/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1217<F: Float>(t1433: F, t641: F, t8513: F, t4017: F, t79: F, t4021: F, t8307: F, t32781: F, t532: F, t1983: F, t6879: F, t33160: F, t6876: F) -> (F, F, F, F, F) {
    let t119971 = t8513 * t641 * t1433;
    let t119975 = t8513 * t79 * t4017;
    let t119990 = t8513 * t8307 * t4021;
    let t119999 = t532 * t32781;
    let t120002 = F::cast_from(3.0_f64) * t1983 * t119999 * t6879;
    let t120008 = F::cast_from(3.0_f64) * t6876 * t33160;
    (t119971, t119975, t119990, t120002, t120008)
}
