//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1221/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1221<F: Float>(t5194: F, t782: F, t5198: F, t213: F, t5187: F, t1307: F, t221: F, t3719: F, t5196: F, t3732: F, t67: F, t792: F) -> (F, F, F, F) {
    let t16081 = t782 * t5194;
    let t16083 = F::new(0.23333333333333333332e-1) * t16081 * t5198;
    let t16084 = t213 * t5187;
    let t16086 = t221 * t16084 * t1307;
    let t16090 = t221 * t5196 * t3719;
    let t16093 = t3732 * t67;
    let t16094 = t792 * t16093;
    (t16083, t16086, t16090, t16094)
}
