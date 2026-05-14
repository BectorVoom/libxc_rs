//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1333/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1333<F: Float>(t11789: F, t820: F, t3577: F, t3579: F, t11737: F, t44857: F, t11791: F, t3490: F, t1227: F, t248: F, t3252: F, t3248: F, t11665: F, t11698: F, t11683: F, t11697: F) -> (F, F, F, F, F, F, F) {
    let t44951 = t820 * t11789;
    let t44953 = t3577 * t44951 * t3579;
    let t44965 = t44857 * t11737;
    let t44968 = t3490 * t11791;
    let t44972 = t1227 * t248 * t11789 * t3252;
    let t44976 = t1227 * t248 * t11789 * t3248;
    let t44982 = t11665 * t11698;
    let t44985 = t3577 * t11697 * t11683;
    (t44953, t44965, t44968, t44972, t44976, t44982, t44985)
}
