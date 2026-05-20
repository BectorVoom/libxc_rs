//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1175/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1175<F: Float>(t4343: F, t4518: F, t3966: F, t978: F, t977: F, t135: F, t1599: F, t973: F, t1597: F, t2987: F) -> (F, F, F, F, F, F) {
    let t4519 = t4518 * t4343;
    let t4522 = t978 * t3966;
    let t4523 = t977 * t4522;
    let t4528 = t135 * t1599;
    let t4529 = t973 * t4528;
    let t4531 = t2987 * t1597;
    (t4519, t4522, t4523, t4528, t4529, t4531)
}
