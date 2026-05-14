//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 500/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk500<F: Float>(t2250: F, t978: F, t977: F, t2822: F, t2824: F, t2828: F, t2831: F, t2834: F, t340: F, t343: F, t974: F, t984: F, t2955: F, t2958: F, t2960: F, t2969: F, t2972: F, t2975: F, t2982: F, t2986: F, t2991: F, t2996: F, t346: F, t973: F, t980: F, t987: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2999 = t978 * t2250;
    let t3000 = t977 * t2999;
    let t3003 = 5.0 / 18.0 * t2822;
    let t3008 = -t3003 - 2.0 / 9.0 * t2824 + t2828 / 18.0 - t2831 / 3.0 + t2834 / 6.0;
    let t3009 = t340 * t3008;
    let t3010 = t3009 * t343;
    let t3011 = t974 * t3010;
    let t3014 = t984 * t984;
    let t3016 = t340 * t3014 * t343;
    let t3017 = t974 * t3016;
    let t3020 = 0.81481481481481481481e-2 * t2955 * t346 - 0.14814814814814814814e-2 * t2958 - 0.14814814814814814814e-2 * t2960 * t980 + 0.44444444444444444444e-2 * t2960 * t987 - t2969 + 0.18518518518518518518e-3 * t2972 - 0.55555555555555555554e-3 * t2975 + 0.37037037037037037036e-3 * t973 * t2982 - 0.55555555555555555554e-3 * t2986 * t2991 - 0.55555555555555555554e-3 * t973 * t2996 + 0.27777777777777777777e-3 * t973 * t3000 - 0.83333333333333333332e-3 * t973 * t3011 - 0.83333333333333333332e-3 * t973 * t3017;
    (t2999, t3000, t3008, t3010, t3011, t3014, t3016, t3017, t3020)
}
