//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 554/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk554<F: Float>(t265: F, t699: F, t305: F, t118: F, t14303: F, t14306: F, t14312: F, t14431: F, t14432: F, t14433: F, t14435: F, t14440: F, t14443: F, t14447: F, t14450: F, t14454: F, t14457: F, t14460: F, t14461: F, t14462: F, t14463: F, t14464: F, t14468: F) -> (F, F, F) {
    let t14469 = t699 * t265;
    let t14470 = t305 * t14469;
    let t14471 = F::new(0.39914139006212695213e-1) * t14470;
    let t14472 = t14431 - t14432 - t14433 - F::new(0.39914139006212695214e-1) * t118 * t14435 - t14440 - t14443 + t14447 - t14450 - t14454 + t14457 + t14460 - t14461 + t14462 - t14463 - t14464 - F::new(0.93188427318671584245e-2) * t14303 + F::new(0.15531404553111930708e-1) * t14306 + F::new(0.31062809106223861415e-2) * t14312 + t14468 + t14471;
    (t14469, t14471, t14472)
}
