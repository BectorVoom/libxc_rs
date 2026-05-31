//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2467/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2467<F: Float>(t11887: F, t44690: F, t42339: F, t466: F, t11715: F, t42341: F, t11721: F, t23508: F, t11714: F, t476: F, t3508: F, t11883: F, t3493: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44691 = t44690 * t11887;
    let t44696 = t466 * t42339;
    let t44698 = t44696 * t42341 * t11715;
    let t44701 = t23508 * t11721;
    let t44722 = F::cast_from(1.0_f64) / t11714 / t476;
    let t44724 = t44696 * t42341 * t44722;
    let t44725 = t3508 * t3508;
    let t44726 = t23508 * t44725;
    let t44730 = t11883 * t3493;
    (t44691, t44696, t44698, t44701, t44722, t44724, t44725, t44726, t44730)
}
