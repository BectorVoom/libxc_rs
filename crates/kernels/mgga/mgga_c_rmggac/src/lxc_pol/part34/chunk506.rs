//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 506/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk506<F: Float>(t14451: F, t321: F, t5259: F, t333: F, t4669: F, t352: F, t5148: F, t14288: F, t14291: F, t14294: F, t14299: F, t14316: F, t265: F, t699: F, t305: F, t118: F, t14303: F, t14306: F, t14312: F, t14431: F, t14432: F, t14433: F, t14435: F, t14440: F, t14443: F, t14447: F, t14450: F) -> (F, F, F, F) {
    let t14452 = t14451 * t321;
    let t14453 = t5259 * t14452;
    let t14454 = 0.2993560425465952141e-1 * t14453;
    let t14455 = t14451 * t333;
    let t14456 = t4669 * t14455;
    let t14457 = 0.44903406381989282115e-1 * t14456;
    let t14458 = t14451 * t352;
    let t14459 = t5148 * t14458;
    let t14460 = 0.2993560425465952141e-1 * t14459;
    let t14461 = 0.18183107769496894487e-1 * t14288;
    let t14462 = 0.20455996240684006296e-1 * t14291;
    let t14463 = 0.40911992481368012592e-1 * t14294;
    let t14464 = 0.10227998120342003148e-1 * t14299;
    let t14468 = 0.68186654135613354325e-2 * t14316;
    let t14469 = t699 * t265;
    let t14470 = t305 * t14469;
    let t14471 = 0.39914139006212695213e-1 * t14470;
    let t14472 = t14431 - t14432 - t14433 - 0.39914139006212695214e-1 * t118 * t14435 - t14440 - t14443 + t14447 - t14450 - t14454 + t14457 + t14460 - t14461 + t14462 - t14463 - t14464 - 0.93188427318671584245e-2 * t14303 + 0.15531404553111930708e-1 * t14306 + 0.31062809106223861415e-2 * t14312 + t14468 + t14471;
    (t14461, t14469, t14471, t14472)
}
