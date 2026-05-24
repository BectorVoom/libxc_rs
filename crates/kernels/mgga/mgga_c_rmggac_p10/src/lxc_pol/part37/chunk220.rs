//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 220/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk220<F: Float>(t1528: F, t196: F, t1004: F, t498: F, t500: F, t589: F, t1022: F, t1023: F, t1050: F, t1087: F, t1094: F, t1104: F, t1112: F, t1133: F, t1140: F, t1143: F, t1424: F, t1425: F, t1429: F, t1430: F, t1434: F, t1437: F, t619: F) -> F {
    let t1529 = t196 * t1528;
    let t1532 = t1004 * t498;
    let t1535 = t500 * t589;
    let t1538 = -t1424 + F::new(0.93273e-1) * t1425 * t1023 + t1429 + F::new(0.186546e0) * t1143 * t1430 - t1050 + F::new(0.31091e-1) * t1529 * t500 + t1133 - F::new(0.31091e-1) * t619 * t1532 - t1094 + t1104 + t1112 - t1087 - t1434 + t1140 + F::new(0.93273e-1) * t1022 * t1535 - t1437;
    t1538
}
