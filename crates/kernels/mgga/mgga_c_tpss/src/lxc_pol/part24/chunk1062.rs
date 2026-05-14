//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1062/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1062<F: Float>(t14477: F, t3923: F, t14482: F, t14487: F, t3919: F, t242: F, t2751: F, t4830: F, t967: F, t4834: F, t14452: F, t970: F, t2675: F, t4978: F, t2722: F, t1407: F, t3950: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14999 = t3923 * t14477;
    let t15002 = t3923 * t14482;
    let t15005 = t3919 * t14487;
    let t15011 = t242 * t2751 * t4830;
    let t15012 = t967 * t15011;
    let t15017 = t242 * t2751 * t4834;
    let t15018 = t967 * t15017;
    let t15021 = t242 * t970 * t14452;
    let t15027 = t242 * t2675 * t4978;
    let t15028 = t2722 * t15027;
    let t15031 = t3950 * t1407;
    (t14999, t15002, t15005, t15011, t15012, t15017, t15018, t15021, t15027, t15028, t15031)
}
