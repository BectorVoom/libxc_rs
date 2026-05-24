//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1061/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1061<F: Float>(t76479: F, t5148: F, t570: F, t71903: F, t321: F, t77970: F, t27101: F, t77204: F, t25854: F, t77786: F, t27048: F, t77789: F) -> (F, F, F, F, F, F) {
    let t78228 = F::cast_from(0.54549323308490683456e-1_f64) * t76479;
    let t78236 = t5148 * t71903 * t570;
    let t78237 = F::cast_from(0.2993560425465952141e-1_f64) * t78236;
    let t78240 = F::cast_from(0.11974241701863808564e0_f64) * t5148 * t77970 * t321;
    let t78244 = t27101 * t77204;
    let t78245 = F::cast_from(0.5987120850931904282e-1_f64) * t78244;
    let t78246 = t25854 * t77786;
    let t78247 = F::cast_from(0.8980681276397856423e-1_f64) * t78246;
    let t78248 = t27048 * t77789;
    (t78228, t78237, t78240, t78245, t78247, t78248)
}
