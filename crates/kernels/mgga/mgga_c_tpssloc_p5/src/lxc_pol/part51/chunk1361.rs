//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1361/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1361<F: Float>(t2018: F, t22574: F, t24432: F, t5187: F, t24995: F, t37790: F, t5308: F, t2314: F, t33617: F, t4034: F, t652: F, t7156: F, t7467: F) -> (F, F, F, F, F) {
    let t120986 = F::cast_from(3.0_f64) * t22574 * t24432 * t2018 * t5187;
    let t120991 = F::cast_from(6.0_f64) * t24995 * t37790 * t5308;
    let t120993 = F::cast_from(2.0_f64) * t2314 * t33617;
    let t120995 = F::cast_from(2.0_f64) * t4034 * t33617;
    let t120998 = F::cast_from(2.0_f64) * t652 * t7156 * t7467;
    (t120986, t120991, t120993, t120995, t120998)
}
