//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1452/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1452<F: Float>(t11584: F, t11589: F, t3447: F, t11153: F, t460: F, t9288: F, t3242: F, t405: F, t974: F, t43763: F, t461: F, t11509: F, t1174: F, t15281: F) -> (F, F, F, F, F, F) {
    let t44602 = t3447 * t11589 * t11584;
    let t44607 = t460 * t11153;
    let t44608 = t44607 * t9288;
    let t44620 = F::new(1.0) / t405 / t3242;
    let t44621 = t974 * t44620;
    let t44622 = t461 * t43763;
    let t44628 = t1174 * t15281 * t11509;
    (t44602, t44608, t44620, t44621, t44622, t44628)
}
