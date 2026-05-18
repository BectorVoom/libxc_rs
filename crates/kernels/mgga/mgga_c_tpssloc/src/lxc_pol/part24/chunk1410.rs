//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1410/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1410<F: Float>(t10121: F, t28: F, t1081: F, t2379: F, t23788: F, t46240: F, t25927: F, t46320: F, t10140: F, t3231: F, t776: F, t2553: F) -> (F, F, F, F, F, F, F) {
    let t83559 = t28 * t10121;
    let t83566 = t1081 * t2379;
    let t83579 = t23788 * t46240;
    let t83582 = t25927 * t46320;
    let t83585 = t28 * t10140;
    let t83592 = t3231 * t776;
    let t83596 = t1081 * t2553;
    (t83559, t83566, t83579, t83582, t83585, t83592, t83596)
}
