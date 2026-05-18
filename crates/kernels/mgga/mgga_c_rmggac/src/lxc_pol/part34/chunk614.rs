//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 614/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk614<F: Float>(t15224: F, t15228: F, t15232: F, t15236: F, t875: F, t9551: F, t1971: F, t3351: F, t2338: F, t702: F, t638: F, t639: F) -> (F, F, F, F, F, F, F, F) {
    let t15484 = F::new(0.17519306092901367188e-6) * t15224;
    let t15485 = F::new(0.15961724959986689775e-4) * t15228;
    let t15486 = F::new(0.1276937996798935182e-4) * t15232;
    let t15487 = F::new(0.2553875993597870364e-4) * t15236;
    let t15488 = t875 * t9551;
    let t15489 = t1971 * t15488;
    let t15490 = t3351 * t15489;
    let t15491 = F::new(0.85129199786595678796e-5) * t15490;
    let t15492 = t2338 * t702;
    let t15494 = t638 * t639 * t15492;
    (t15484, t15485, t15486, t15487, t15489, t15491, t15492, t15494)
}
