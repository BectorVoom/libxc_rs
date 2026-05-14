//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 910/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk910<F: Float>(t1755: F, t7327: F, t1090: F, t7376: F, t8034: F, t7377: F, t24833: F, t8073: F, t5068: F, t8082: F, t5079: F, t221: F, t4899: F, t2127: F) -> (F, F, F, F, F, F, F, F) {
    let t27531 = t7327 * t1755;
    let t27532 = t7376 * t1090;
    let t27533 = t27531 * t27532;
    let t27536 = t8034 * t7327;
    let t27537 = t27536 * t7377;
    let t27540 = t24833 * t8073;
    let t27543 = t8082 * t5068;
    let t27546 = t8082 * t5079;
    let t27548 = t221 * t4899;
    let t27549 = t2127 * t27548;
    (t27532, t27533, t27536, t27537, t27540, t27543, t27546, t27549)
}
