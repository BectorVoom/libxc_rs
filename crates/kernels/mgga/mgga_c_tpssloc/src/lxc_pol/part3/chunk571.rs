//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 571/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk571<F: Float>(t218: F, t2710: F, t225: F, t853: F, t257: F, t856: F, t68: F, t865: F, t252: F, t2627: F, t2633: F, t814: F, t852: F) -> (F, F, F, F, F, F, F) {
    let t2711 = t218 * t2710;
    let t2713 = t853 * t225;
    let t2717 = F::new(1.0) / t856 / t257;
    let t2718 = t68 * t2717;
    let t2719 = t865 * t865;
    let t2720 = t2718 * t2719;
    let t2728 = t2627 * t252;
    let t2729 = t2728 * t2633;
    let t2732 = t814 * t852;
    (t2711, t2713, t2718, t2719, t2720, t2729, t2732)
}
