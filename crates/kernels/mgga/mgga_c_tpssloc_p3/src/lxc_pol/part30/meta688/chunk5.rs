//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2190/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2190<F: Float>(t28237: F, t532: F, t1983: F, t6879: F, t510: F, t652: F, t96729: F, t1874: F, t96683: F, t25992: F, t7685: F, t25985: F) -> (F, F, F, F, F) {
    let t97817 = t532 * t28237;
    let t97820 = F::cast_from(3.0_f64) * t1983 * t97817 * t6879;
    let t97829 = F::cast_from(2.0_f64) * t652 * t510 * t96729;
    let t97831 = F::cast_from(4.0_f64) * t96683 * t1874;
    let t97833 = F::cast_from(2.0_f64) * t7685 * t25992;
    let t97835 = F::cast_from(6.0_f64) * t7685 * t25985;
    (t97820, t97829, t97831, t97833, t97835)
}
