//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1008/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1008<F: Float>(t27101: F, t78220: F, t14444: F, t1627: F, t25854: F, t76479: F, t5148: F, t570: F, t71903: F, t321: F, t77970: F, t77204: F) -> (F, F, F, F, F, F, F) {
    let t78222 = F::new(0.23948483403727617128e0) * t27101 * t78220;
    let t78223 = t14444 * t1627;
    let t78225 = F::new(0.35922725105591425692e0) * t25854 * t78223;
    let t78228 = F::new(0.54549323308490683456e-1) * t76479;
    let t78236 = t5148 * t71903 * t570;
    let t78237 = F::new(0.2993560425465952141e-1) * t78236;
    let t78240 = F::new(0.11974241701863808564e0) * t5148 * t77970 * t321;
    let t78244 = t27101 * t77204;
    (t78222, t78223, t78225, t78228, t78237, t78240, t78244)
}
