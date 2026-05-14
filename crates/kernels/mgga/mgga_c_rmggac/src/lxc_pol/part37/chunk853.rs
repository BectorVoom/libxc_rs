//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 853/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk853<F: Float>(t14444: F, t1624: F, t27101: F, t1627: F, t25854: F, t76479: F, t5148: F, t570: F, t71903: F, t321: F, t77970: F, t77204: F, t77786: F, t27048: F, t77789: F, t27055: F, t77341: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t78220 = t14444 * t1624;
    let t78222 = 0.23948483403727617128e0 * t27101 * t78220;
    let t78223 = t14444 * t1627;
    let t78225 = 0.35922725105591425692e0 * t25854 * t78223;
    let t78228 = 0.54549323308490683456e-1 * t76479;
    let t78236 = t5148 * t71903 * t570;
    let t78237 = 0.2993560425465952141e-1 * t78236;
    let t78240 = 0.11974241701863808564e0 * t5148 * t77970 * t321;
    let t78244 = t27101 * t77204;
    let t78245 = 0.5987120850931904282e-1 * t78244;
    let t78246 = t25854 * t77786;
    let t78247 = 0.8980681276397856423e-1 * t78246;
    let t78248 = t27048 * t77789;
    let t78249 = 0.8980681276397856423e-1 * t78248;
    let t78251 = 0.35922725105591425692e0 * t27055 * t77341;
    (t78220, t78222, t78223, t78225, t78228, t78237, t78240, t78245, t78247, t78249, t78251)
}
