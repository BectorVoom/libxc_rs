//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1234/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1234<F: Float>(t136: F, t76624: F, t908: F, t2826: F, t76608: F, t76612: F, t76616: F, t76620: F, t43002: F, t48103: F, t60168: F, t60173: F, t60204: F, t68452: F, t68454: F, t13769: F, t17794: F, t17804: F, t2986: F, t340: F, t343: F, t4510: F, t4531: F, t61310: F, t61313: F, t69548: F, t69647: F, t69683: F, t69686: F, t69691: F, t69699: F, t69727: F, t69739: F, t69746: F, t76593: F, t76901: F, t973: F, t974: F) -> (F, F, F, F, F, F) {
    let t76903 = t136 * t908 * t76624;
    let t76906 = t136 * t2826 * t76608;
    let t76909 = t136 * t908 * t76612;
    let t76912 = t136 * t908 * t76616;
    let t76915 = t136 * t908 * t76620;
    let t76922 = -4.0 / 3.0 * t76903 + 2.0 / 9.0 * t76906 - 4.0 * t76909 + 6.0 * t76912 - t76915 - 20.0 / 9.0 * t60168 + 10.0 / 9.0 * t60173 + 8.0 / 3.0 * t68452 - t43002 - 4.0 / 9.0 * t68454 - 160.0 / 81.0 * t48103 + 10.0 / 27.0 * t60204;
    let t76943 = -0.16666666666666666666e-2 * t2986 * t17804 * t17794 - 0.13333333333333333333e-1 * t2986 * t4510 * t76593 + 0.88888888888888888886e-2 * t2986 * t13769 * t69548 - 0.83333333333333333332e-3 * t973 * t974 * t340 * (t76901 + t76922) * t343 - 0.22222222222222222221e-2 * t69683 - 0.11111111111111111111e-2 * t69686 - 0.11111111111111111111e-2 * t69691 - 0.14814814814814814815e-2 * t69699 - 0.29629629629629629628e-2 * t69727 + 0.37037037037037037036e-3 * t69739 + 0.66666666666666666664e-2 * t2986 * t4531 * t69746 - 0.44444444444444444444e-2 * t2986 * t13769 * t69647 + 0.11111111111111111111e-2 * t61310 + 0.11111111111111111111e-2 * t61313;
    (t76903, t76906, t76909, t76912, t76915, t76943)
}
