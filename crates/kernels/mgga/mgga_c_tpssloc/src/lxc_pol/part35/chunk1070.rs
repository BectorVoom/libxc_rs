//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1070/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1070<F: Float>(t1419: F, t55: F, t1240: F, t1760: F, t2122: F, t24574: F, t8003: F, t6686: F, t8020: F) -> (F, F, F, F, F) {
    let t27356 = t1419 * t55;
    let t27381 = t1240 * t1760;
    let t27382 = t2122 * t27381;
    let t27401 = t24574 * t8003;
    let t27406 = t8020 * t6686;
    (t27356, t27381, t27382, t27401, t27406)
}
