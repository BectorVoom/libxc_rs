//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 585/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk585<F: Float>(t2793: F, t913: F, t2792: F, t273: F, t276: F, t896: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F, t894: F, t901: F, t241: F, t63: F, t281: F, t283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2794 = t2793 * t913;
    let t2796 = 2.0 * t2792 * t2794;
    let t2798 = 1.0 / t276 / t273;
    let t2799 = t896 * t896;
    let t2800 = t2798 * t2799;
    let t2802 = 4.0 / 9.0 * t2764;
    let t2807 = t2802 + 2.0 / 9.0 * t2766 - 2.0 / 9.0 * t2773 + 2.0 / 3.0 * t2778 - t2782 / 3.0;
    let t2808 = t894 * t2807;
    let t2810 = 0.39862222222222222223e0 * t2764;
    let t2815 = 1.0/f64::sqrt(t273);
    let t2816 = t2815 * t2799;
    let t2818 = t901 * t2807;
    let t2820 = t63 * t241;
    let t2822 = t281 * t2820 * t283;
    (t2794, t2796, t2798, t2799, t2800, t2802, t2807, t2808, t2810, t2815, t2816, t2818, t2820, t2822)
}
