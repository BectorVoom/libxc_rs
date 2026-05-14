//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1072/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1072<F: Float>(t1119: F, t14845: F, t3308: F, t4740: F, t1657: F, t3312: F, t3316: F, t11282: F, t1694: F, t11285: F, t3377: F, t1164: F, t300: F, t4832: F, t1166: F, t3419: F, t4869: F) -> (F, F, F, F, F, F) {
    let t14847 = 2.0 * t14845 * t1119;
    let t14849 = 1.0 * t4740 * t3308;
    let t14850 = t1657 * t3312;
    let t14852 = 0.16081979498692535067e2 * t14850 * t3316;
    let t14853 = t11282 * t1694;
    let t14854 = t11285 * t3377;
    let t14855 = t14853 * t14854;
    let t14857 = 0.10254018858216406658e4 * t1164 * t14855;
    let t14858 = t300 * t4832;
    let t14860 = 0.11696447245269292414e1 * t14858 * t1166;
    let t14862 = 0.5848223622634646207e0 * t4869 * t3419;
    (t14847, t14849, t14852, t14857, t14860, t14862)
}
