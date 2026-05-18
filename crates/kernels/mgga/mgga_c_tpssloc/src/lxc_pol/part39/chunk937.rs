//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 937/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk937<F: Float>(t9798: F, t9860: F, t157: F, t153: F, t2371: F, t2531: F, t2528: F, t2517: F, t607: F, t707: F, t2652: F, t2663: F) -> (F, F, F, F, F, F) {
    let t9861 = t9798 + t9860;
    let t9862 = t157 * t9861;
    let t9863 = t153 * t9862;
    let t9864 = t2531 * t2371;
    let t9866 = t2531 * t2528;
    let t9868 = t2517 * t607;
    let t9869 = t707 * t9868;
    let t9871 = t2652 * t2663;
    (t9861, t9863, t9864, t9866, t9869, t9871)
}
