//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1229/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1229<F: Float>(t21303: F, t49274: F, t10704: F, t42028: F, t76644: F, t21239: F, t4488: F, t959: F, t5950: F, t5919: F, t5943: F, t10165: F, t1052: F, t1634: F, t1635: F, t17588: F, t18074: F, t21662: F, t21663: F, t21677: F, t21692: F, t3174: F, t388: F, t43604: F, t4557: F, t4660: F, t5848: F, t5914: F, t5920: F, t69871: F, t70978: F, t70980: F) -> (F, F, F, F, F) {
    let t76668 = 0.2069040516770936012e4 * t49274 * t21303;
    let t76671 = 0.62071215503128080361e4 * t42028 * t76644 * t10704;
    let t76674 = 0.46785788981077169656e1 * t959 * t4488 * t21239;
    let t76675 = t5950 * t5950;
    let t76684 = t5919 * t5919;
    let t76706 = t5943 * t5943;
    let t76715 = -36.0 * t10165 * t1052 * t5919 * t5943 + 8.0 * t1052 * t1634 * t21662 * t3174 + 6.0 * t1052 * t3174 * t76706 + 24.0 * t1052 * t43604 * t76684 + 6.0 * t388 * t5848 * t5914 - 4.0 * t1635 * t69871 - 4.0 * t1635 * t70978 - 12.0 * t1635 * t70980 + 24.0 * t17588 * t5920 + 12.0 * t18074 * t5920 - 4.0 * t21663 * t4660 - 24.0 * t21677 * t4557 + 24.0 * t21692 * t4557;
    (t76668, t76671, t76674, t76675, t76715)
}
