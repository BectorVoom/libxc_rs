//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 570/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk570<F: Float>(t290: F, t2793: F, t2842: F, t2764: F, t2766: F, t2773: F, t2778: F, t2782: F, t919: F, t923: F, t307: F, t922: F, t302: F, t931: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t2843 = t290 * t290;
    let t2844 = 1.0 / t2843;
    let t2845 = t2793 * t2844;
    let t2847 = 0.16081979498692535067e2 * t2842 * t2845;
    let t2848 = 0.22831111111111111111e-1 * t2764;
    let t2853 = t2848 + 0.11415555555555555555e-1 * t2766 - 0.11415555555555555555e-1 * t2773 + 0.34246666666666666666e-1 * t2778 - 0.17123333333333333333e-1 * t2782;
    let t2856 = t919 * t923;
    let t2859 = t922 * t307;
    let t2860 = 1.0 / t2859;
    let t2861 = t302 * t2860;
    let t2862 = t931 * t931;
    (t2843, t2844, t2845, t2847, t2848, t2853, t2856, t2860, t2861, t2862)
}
