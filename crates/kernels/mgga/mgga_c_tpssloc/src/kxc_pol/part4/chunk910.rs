//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 910/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk910<F: Float>(t16752: F, t232: F, t860: F, t2732: F, t5612: F, t1509: F, t1519: F, t829: F, t4234: F, t4282: F, t5550: F, t9573: F, t213: F, t5527: F, t221: F, t776: F) -> (F, F, F, F, F, F, F, F) {
    let t16753 = t16752 * t232;
    let t16754 = t860 * t16753;
    let t16756 = t2732 * t5612;
    let t16758 = t1519 * t1509;
    let t16759 = t16758 * t829;
    let t16762 = t4282 * t4234;
    let t16769 = t9573 * t5550;
    let t16771 = t213 * t5527;
    let t16773 = t221 * t16771 * t776;
    (t16753, t16754, t16756, t16758, t16759, t16762, t16769, t16773)
}
