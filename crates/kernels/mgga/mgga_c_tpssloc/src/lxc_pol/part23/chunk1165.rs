//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1165/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1165<F: Float>(t1734: F, t6218: F, t1222: F, t22169: F, t11539: F, t1174: F, t21745: F, t1213: F, t22244: F, t248: F, t3570: F, t1227: F, t21758: F, t45268: F, t11692: F, t11697: F, t22283: F) -> (F, F, F, F, F, F) {
    let t72767 = t6218 * t1734;
    let t72798 = t22169 * t1222;
    let t72815 = t1174 * t11539 * t21745;
    let t72849 = t1213 * t248 * t3570 * t22244;
    let t72857 = t1227 * t248 * t45268 * t21758;
    let t72864 = t11692 * t11697 * t22283;
    (t72767, t72798, t72815, t72849, t72857, t72864)
}
