//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1046/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1046<F: Float>(t14577: F, t1629: F, t4673: F, t4680: F, t1049: F, t4649: F, t1060: F, t11066: F, t3040: F, t1615: F, t3166: F, t4677: F, t1625: F, t3120: F, t14506: F, t3199: F) -> (F, F, F, F, F, F, F, F) {
    let t14578 = t1629 * t14577;
    let t14581 = t4680 * t4673;
    let t14586 = t1049 * t4649;
    let t14587 = t14586 * t1060;
    let t14590 = t11066 * t3040;
    let t14591 = t1629 * t14590;
    let t14595 = t3166 * t1615;
    let t14596 = t14595 * t1060;
    let t14600 = t4677 * t4673;
    let t14605 = t1625 * t3120;
    let t14606 = t14605 * t1060;
    let t14608 = t14506 * t3199;
    (t14578, t14581, t14587, t14591, t14596, t14600, t14606, t14608)
}
