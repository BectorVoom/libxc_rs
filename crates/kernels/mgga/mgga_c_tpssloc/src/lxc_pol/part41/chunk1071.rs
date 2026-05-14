//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1071/1183 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1071<F: Float>(t14850: F, t4786: F, t1117: F, t5989: F, t3313: F, t1671: F, t4781: F, t3264: F, t6024: F, t11190: F, t1098: F, t5983: F, t1119: F, t14845: F, t4740: F, t4782: F) -> (F, F, F, F, F, F, F) {
    let t18676 = 0.32163958997385070134e2 * t14850 * t4786;
    let t18677 = t5989 * t1117;
    let t18679 = 6.0 * t3313 * t18677;
    let t18680 = t1671 * t4781;
    let t18682 = 4.0 * t3264 * t18680;
    let t18683 = t6024 * t1117;
    let t18685 = 0.96491876992155210402e2 * t11190 * t18683;
    let t18686 = t5983 * t1098;
    let t18688 = 1.0 * t18686 * t1119;
    let t18690 = 2.0 * t14845 * t1671;
    let t18692 = 2.0 * t4740 * t4782;
    (t18676, t18679, t18682, t18685, t18688, t18690, t18692)
}
