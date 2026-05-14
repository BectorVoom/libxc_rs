//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 850/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk850<F: Float>(t21826: F, t449: F, t300: F, t18910: F, t4861: F, t1164: F, t4874: F, t6085: F, t1695: F, t6084: F, t1694: F, t18615: F, t1683: F, t6052: F, t1682: F, t18643: F) -> (F, F, F, F, F, F, F, F) {
    let t21827 = t21826 * t449;
    let t21829 = 0.19751673498613801407e-1 * t300 * t21827;
    let t21830 = t18910 * t4861;
    let t21832 = 0.51947577317044391277e2 * t1164 * t21830;
    let t21833 = t4874 * t6085;
    let t21835 = 0.35089341735807877242e1 * t1164 * t21833;
    let t21836 = t1695 * t6084;
    let t21839 = t18615 * t1694;
    let t21842 = t1683 * t6052;
    let t21845 = t18643 * t1682;
    (t21827, t21829, t21832, t21835, t21836, t21839, t21842, t21845)
}
