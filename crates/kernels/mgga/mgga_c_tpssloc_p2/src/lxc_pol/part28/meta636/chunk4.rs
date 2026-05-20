//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2024/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2024<F: Float>(t91356: F, t91358: F, t91364: F, t91386: F, t80889: F, t80915: F, t84533: F, t84536: F, t91354: F, t91362: F, t91366: F, t91370: F, t91374: F, t91378: F, t91381: F, t91384: F, t91389: F, t91391: F) -> F {
    let t93742 = F::cast_from(0.33913115119077928316e-1_f64) * t91356;
    let t93743 = F::cast_from(0.56521858531796547194e-2_f64) * t91358;
    let t93745 = F::new(7.0) / F::new(144.0) * t91364;
    let t93753 = F::new(35.0) / F::new(144.0) * t91386;
    let t93756 = -t84533 - F::cast_from(0.11869590291677274911e0_f64) * t80889 - F::cast_from(0.96894614625936938048e-2_f64) * t91354 - t93742 + t93743 - t91362 / F::new(128.0) - t93745 - t84536 - t91366 / F::new(24.0) - F::cast_from(0.24223653656484234512e-2_f64) * t91370 - F::cast_from(0.40372756094140390853e-3_f64) * t91374 + F::cast_from(0.80745512188280781706e-3_f64) * t91378 + F::cast_from(0.16149102437656156341e-2_f64) * t91381 - F::new(119.0) / F::new(1728.0) * t80915 - t91384 / F::new(768.0) - t93753 + t91389 / F::new(384.0) - t91391 / F::new(768.0);
    t93756
}
