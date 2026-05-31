//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2080/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2080<F: Float>(t22949: F, t7685: F, t25010: F, t6876: F, t1307: F, t19577: F, t24995: F, t8643: F, t1983: F, t22584: F, t26167: F, t12725: F, t6535: F) -> (F, F, F, F, F) {
    let t86682 = t7685 * t22949;
    let t86684 = F::cast_from(2.0_f64) * t6876 * t25010;
    let t86685 = t19577 * t1307;
    let t86688 = F::cast_from(12.0_f64) * t24995 * t8643 * t86685;
    let t86693 = F::cast_from(3.0_f64) * t1983 * t26167 * t22584;
    let t86698 = F::cast_from(4.0_f64) * t12725 * t6535;
    (t86682, t86684, t86688, t86693, t86698)
}
