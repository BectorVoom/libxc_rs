//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1515/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1515<F: Float>(t12189: F, t1804: F, t5194: F, t782: F, t5198: F, t213: F, t5187: F, t3732: F, t67: F, t792: F, t1799: F, t212: F) -> (F, F, F, F, F, F, F) {
    let t16078 = t12189 * t1804;
    let t16081 = t782 * t5194;
    let t16083 = F::cast_from(0.23333333333333333332e-1_f64) * t16081 * t5198;
    let t16084 = t213 * t5187;
    let t16093 = t3732 * t67;
    let t16094 = t792 * t16093;
    let t16095 = t212 * t1799;
    (t16078, t16081, t16083, t16084, t16093, t16094, t16095)
}
