//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 193/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk193<F: Float>(t570: F, t874: F, t352: F, t321: F, t559: F, t235: F, t837: F, t333: F, t338: F, t558: F, t171: F, t577: F, t433: F, t521: F, t983: F, t437: F, t50: F) -> (F, F, F, F, F, F, F, F) {
    let t1357 = t874 * t570;
    let t1358 = t1357 * t352;
    let t1361 = t559 * t321;
    let t1364 = t235 * t837;
    let t1365 = t559 * t333;
    let t1368 = t338 * t558;
    let t1369 = t1368 * t352;
    let t1372 = t577 * t171;
    let t1373 = t1372 * t433;
    let t1374 = 0.5848223622634646207e0 * t1373;
    let t1375 = t983 * t521;
    let t1378 = t437 * t50;
    (t1358, t1361, t1364, t1365, t1369, t1374, t1375, t1378)
}
