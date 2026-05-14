//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 811/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk811<F: Float>(t2617: F, t2696: F, t849: F, t820: F, t847: F, t9516: F, t2645: F, t2647: F, t9621: F, t2618: F, t2623: F, t2630: F, t2635: F, t2643: F, t2681: F, t2703: F, t843: F, t9967: F, t9974: F, t9978: F, t9983: F, t9986: F, t9988: F, t9990: F) -> (F, F, F, F) {
    let t9993 = t2617 * t2696;
    let t9994 = t9993 * t849;
    let t9997 = t847 * t820 * t9516;
    let t10003 = t2645 * t9621 * t2647;
    let t10006 = -t2618 * t2681 / 1024.0 + t9967 * t2635 / 512.0 - t9974 * t9978 / 512.0 + t2630 * t9983 / 512.0 + 7.0 / 1536.0 * t9986 - 35.0 / 384.0 * t9988 - t9990 * t849 / 256.0 + 7.0 / 192.0 * t9994 - t843 * t9997 / 768.0 + 5.0 / 256.0 * t2623 * t2703 + t2643 * t10003 / 256.0;
    (t9993, t9997, t10003, t10006)
}
