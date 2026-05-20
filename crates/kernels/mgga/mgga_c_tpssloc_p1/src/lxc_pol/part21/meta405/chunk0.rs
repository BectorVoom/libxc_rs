//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1890/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1890<F: Float>(t4680: F, t4684: F, t11060: F, t3040: F, t1629: F, t4673: F, t1049: F, t4649: F, t1060: F, t11066: F, t1615: F, t3166: F) -> (F, F, F, F, F, F, F, F) {
    let t14574 = t4680 * t4684;
    let t14577 = t11060 * t3040;
    let t14578 = t1629 * t14577;
    let t14581 = t4680 * t4673;
    let t14586 = t1049 * t4649;
    let t14587 = t14586 * t1060;
    let t14590 = t11066 * t3040;
    let t14591 = t1629 * t14590;
    let t14595 = t3166 * t1615;
    (t14574, t14577, t14578, t14581, t14587, t14590, t14591, t14595)
}
