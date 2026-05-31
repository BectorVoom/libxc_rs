//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1358/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1358<F: Float>(t31759: F, t7685: F, t31300: F, t91655: F, t2018: F, t22574: F, t24432: F, t5187: F, t24995: F, t37790: F, t5308: F, t2314: F, t33617: F) -> (F, F, F, F, F) {
    let t120975 = F::cast_from(3.0_f64) * t7685 * t31759;
    let t120979 = F::cast_from(3.0_f64) * t91655 * t31300;
    let t120986 = F::cast_from(3.0_f64) * t22574 * t24432 * t2018 * t5187;
    let t120991 = F::cast_from(6.0_f64) * t24995 * t37790 * t5308;
    let t120993 = F::cast_from(2.0_f64) * t2314 * t33617;
    (t120975, t120979, t120986, t120991, t120993)
}
