//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2086/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2086<F: Float>(t87405: F, t87432: F, t87653: F, t87666: F, t87718: F, t87779: F, t87898: F, t87915: F, t90503: F, t90551: F, t90582: F, t90642: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t92677 = F::cast_from(0.10541775202358879834e-2_f64) * t87405;
    let t92689 = F::cast_from(0.22608743412718618878e-1_f64) * t87432;
    let t92781 = F::cast_from(0.16449340668482264365e-1_f64) * t87653;
    let t92794 = F::cast_from(0.12793931631041761173e0_f64) * t87666;
    let t92817 = F::cast_from(0.10417915756705434098e0_f64) * t87718;
    let t92863 = F::cast_from(0.16449340668482264365e-1_f64) * t87779;
    let t92954 = F::cast_from(0.52089578783527170489e-1_f64) * t87898;
    let t92961 = F::cast_from(0.16449340668482264365e-1_f64) * t87915;
    let t93335 = F::cast_from(0.12793931631041761173e0_f64) * t90503;
    let t93368 = F::cast_from(0.10417915756705434098e0_f64) * t90551;
    let t93387 = F::cast_from(0.52089578783527170489e-1_f64) * t90582;
    let t93438 = F::cast_from(0.16449340668482264365e-1_f64) * t90642;
    (t92677, t92689, t92781, t92794, t92817, t92863, t92954, t92961, t93335, t93368, t93387, t93438)
}
