//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2102/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2102<F: Float>(t87779: F, t87898: F, t87915: F, t90503: F, t90551: F, t90582: F, t90642: F, t90659: F, t90663: F, t90837: F, t90868: F, t90900: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t92863 = F::cast_from(0.16449340668482264365e-1_f64) * t87779;
    let t92954 = F::cast_from(0.52089578783527170489e-1_f64) * t87898;
    let t92961 = F::cast_from(0.16449340668482264365e-1_f64) * t87915;
    let t93335 = F::cast_from(0.12793931631041761173e0_f64) * t90503;
    let t93368 = F::cast_from(0.10417915756705434098e0_f64) * t90551;
    let t93387 = F::cast_from(0.52089578783527170489e-1_f64) * t90582;
    let t93438 = F::cast_from(0.16449340668482264365e-1_f64) * t90642;
    let t93445 = F::cast_from(0.12793931631041761173e0_f64) * t90659;
    let t93446 = F::cast_from(0.16449340668482264365e-1_f64) * t90663;
    let t93517 = F::cast_from(0.10417915756705434098e0_f64) * t90837;
    let t93538 = F::cast_from(0.12793931631041761173e0_f64) * t90868;
    let t93563 = F::cast_from(0.52089578783527170489e-1_f64) * t90900;
    (t92863, t92954, t92961, t93335, t93368, t93387, t93438, t93445, t93446, t93517, t93538, t93563)
}
