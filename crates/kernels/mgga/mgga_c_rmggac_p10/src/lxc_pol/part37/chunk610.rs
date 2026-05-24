//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 610/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk610<F: Float>(t15456: F, t1971: F, t3351: F, t15197: F, t2347: F, t699: F, t1550: F, t2350: F, t903: F, t2211: F, t2392: F, t739: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15457 = t1971 * t15456;
    let t15458 = t3351 * t15457;
    let t15459 = F::cast_from(0.12769379967989351819e-4_f64) * t15458;
    let t15460 = F::cast_from(0.10227998120342003148e-1_f64) * t15197;
    let t15464 = t699 * t2347;
    let t15465 = t1550 * t15464;
    let t15466 = F::cast_from(0.2993560425465952141e-1_f64) * t15465;
    let t15467 = t699 * t2350;
    let t15468 = t903 * t15467;
    let t15469 = F::cast_from(0.44903406381989282115e-1_f64) * t15468;
    let t15470 = t2211 * t2392;
    let t15471 = t739 * t15470;
    (t15457, t15459, t15460, t15464, t15466, t15467, t15469, t15470, t15471)
}
