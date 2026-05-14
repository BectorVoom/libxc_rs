//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1005/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1005<F: Float>(t11040: F, t847: F, t11026: F, t861: F, t141: F, t11013: F, t2515: F, t3800: F, t673: F, t3797: F, t10990: F, t10992: F, t10994: F, t8647: F, t8661: F, t8665: F) -> (F, F, F, F, F, F) {
    let t11041 = t847 * t11040;
    let t11043 = t861 * t11026;
    let t11044 = t141 * t11043;
    let t11046 = t2515 * t11013;
    let t11047 = t141 * t11046;
    let t11049 = t673 * t3800;
    let t11050 = 0.21908444444444444444e0 * t11049;
    let t11051 = t673 * t3797;
    let t11053 = t10990 - 0.82156666666666666667e-1 * t10992 - 0.91285185185185185185e-1 * t10994 - 0.10954222222222222222e0 * t8647 - t8661 - t8665 + 0.1898925e1 * t11041 - 0.49293999999999999999e0 * t11044 + 0.16431333333333333333e0 * t11047 - t11050 + 0.36514074074074074074e-1 * t11051;
    (t11041, t11044, t11047, t11049, t11051, t11053)
}
