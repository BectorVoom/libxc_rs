//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 589/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk589<F: Float>(t4496: F, t4497: F, t959: F, t1592: F, t2970: F, t973: F, t2978: F, t60: F, t344: F, t4338: F, t1409: F, t2989: F, t2988: F, t2987: F, t4343: F, t3966: F, t978: F) -> (F, F, F, F, F, F, F) {
    let t4498 = t4496 * t4497;
    let t4500 = 0.17315859105681463759e2 * t959 * t4498;
    let t4506 = t2970 * t1592;
    let t4507 = t973 * t4506;
    let t4509 = t60 * t2978;
    let t4510 = t4509 * t344;
    let t4511 = t4510 * t4338;
    let t4514 = t2989 * t1409;
    let t4515 = t2988 * t4514;
    let t4518 = t2987 * t344;
    let t4519 = t4518 * t4343;
    let t4522 = t978 * t3966;
    (t4500, t4507, t4509, t4511, t4515, t4519, t4522)
}
