//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 524/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk524<F: Float>(t252: F, t2627: F, t2633: F, t814: F, t852: F, t829: F, t2679: F, t860: F, t2684: F, t235: F, t2710: F, t226: F, t255: F, t2613: F, t2617: F, t808: F, t812: F, t861: F, t863: F) -> (F, F, F, F, F, F, F, F) {
    let t2728 = t2627 * t252;
    let t2729 = t2728 * t2633;
    let t2732 = t814 * t852;
    let t2733 = t2732 * t829;
    let t2736 = t860 * t2679;
    let t2738 = t860 * t2684;
    let t2740 = t235 * t2710;
    let t2742 = t226 * t2740 + t255 * t2613 - F::new(2.0) * t2617 * t861 + F::new(2.0) * t2729 * t812 - F::new(2.0) * t2733 * t812 - t2736 * t812 - t2738 * t812 + F::new(2.0) * t808 * t863;
    (t2728, t2729, t2732, t2733, t2736, t2738, t2740, t2742)
}
