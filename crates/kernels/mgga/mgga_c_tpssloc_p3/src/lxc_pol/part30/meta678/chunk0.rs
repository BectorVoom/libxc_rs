//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2120/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2120<F: Float>(t1873: F, t96709: F, t5464: F, t81442: F, t666: F, t81446: F, t1453: F, t4067: F, t22473: F, t22470: F, t5488: F, t19529: F, t6530: F) -> (F, F, F, F, F, F, F) {
    let t96711 = F::new(2.0) * t96709 * t1873;
    let t96713 = t81442 * t5464;
    let t96715 = t5464 * t666;
    let t96716 = t81446 * t96715;
    let t96718 = t1453 * t4067;
    let t96719 = t22473 * t96718;
    let t96721 = t22470 * t5488;
    let t96723 = t5488 * t666;
    let t96724 = t22473 * t96723;
    let t96726 = t6530 * t19529;
    (t96711, t96713, t96716, t96719, t96721, t96724, t96726)
}
