//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1846/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1846<F: Float>(t87247: F, t87255: F, t87262: F, t87270: F, t87272: F, t87291: F, t87293: F, t87300: F, t87308: F, t87328: F, t87330: F, t87332: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t92599 = F::new(7.0) / F::new(576.0) * t87247;
    let t92603 = F::new(7.0) / F::new(576.0) * t87255;
    let t92607 = F::new(7.0) / F::new(576.0) * t87262;
    let t92614 = F::new(7.0) / F::new(144.0) * t87270;
    let t92615 = F::new(7.0) / F::new(576.0) * t87272;
    let t92626 = F::new(7.0) / F::new(36.0) * t87291;
    let t92627 = F::cast_from(0.33913115119077928316e-1_f64) * t87293;
    let t92630 = F::new(35.0) / F::new(144.0) * t87300;
    let t92635 = F::cast_from(0.33913115119077928316e-1_f64) * t87308;
    let t92645 = F::cast_from(0.80745512188280781706e-3_f64) * t87328;
    let t92646 = F::new(7.0) / F::new(144.0) * t87330;
    let t92647 = F::new(7.0) / F::new(144.0) * t87332;
    (t92599, t92603, t92607, t92614, t92615, t92626, t92627, t92630, t92635, t92645, t92646, t92647)
}
