//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1856/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1856<F: Float>(t91310: F, t91327: F, t91344: F, t91356: F, t91358: F, t91364: F, t91386: F, t91402: F, t91404: F, t91064: F, t91076: F, t90723: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t93722 = F::cast_from(0.13457585364713463618e-3_f64) * t91310;
    let t93731 = F::cast_from(0.80745512188280781706e-3_f64) * t91327;
    let t93736 = F::cast_from(0.56521858531796547194e-2_f64) * t91344;
    let t93742 = F::cast_from(0.33913115119077928316e-1_f64) * t91356;
    let t93743 = F::cast_from(0.56521858531796547194e-2_f64) * t91358;
    let t93745 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t91364;
    let t93753 = F::cast_from(35.0_f64) / F::cast_from(144.0_f64) * t91386;
    let t93762 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t91402;
    let t93763 = F::cast_from(0.33913115119077928316e-1_f64) * t91404;
    let t93792 = F::cast_from(0.15352717957250113407e0_f64) * t91064;
    let t93794 = F::cast_from(0.76763589786250567036e-1_f64) * t91076;
    let t93824 = F::cast_from(0.16449340668482264365e-1_f64) * t90723;
    (t93722, t93731, t93736, t93742, t93743, t93745, t93753, t93762, t93763, t93792, t93794, t93824)
}
