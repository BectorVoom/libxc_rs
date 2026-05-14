//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1302/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1302<F: Float>(t3311: F, t409: F, t3314: F, t43970: F, t11185: F, t11427: F, t1118: F, t11190: F, t3307: F, t3264: F, t3313: F, t3315: F, t11399: F, t3403: F, t11297: F, t11303: F, t11310: F, t11361: F, t11365: F, t11430: F, t11434: F, t11437: F, t1155: F, t1157: F, t3376: F, t3377: F, t3378: F, t3395: F, t3401: F, t3404: F, t43956: F, t43958: F, t43961: F, t43963: F, t43966: F, t43973: F, t43979: F, t43984: F, t43989: F, t43994: F) -> (F, F, F, F, F, F) {
    let t44073 = t3311 * t3311;
    let t44075 = t409 / t44073;
    let t44076 = t3314 * t3314;
    let t44077 = 1.0 / t44076;
    let t44080 = 0.24955700379505800916e5 * t44075 * t43970 * t44077;
    let t44082 = 24.0 * t11185 * t11427;
    let t44085 = 24.0 * t11190 * t43970 * t1118;
    let t44086 = t3307 * t3307;
    let t44089 = 6.0 * t3264 * t44086 * t1118;
    let t44092 = 0.48245938496077605201e2 * t3313 * t44086 * t3315;
    let t44106 = t11399 * t3403;
    let t44115 = -0.14035736694323150897e2 * t11297 * t11430 + 0.21053605041484726346e2 * t3401 * t3378 * t3395 - 0.46785788981077169656e1 * t3376 * t1157 * t11399 - 0.62337092780453269531e3 * t11365 * t3404 * t3395 + 0.2077903092681775651e3 * t11361 * t11434 + 0.69263436422725855036e2 * t3401 * t44106 * t1155 + 0.61524113149298439947e4 * t11310 * t43984 * t3377 - 24.0 * t11303 * t11437 - t43956 - t43958 - t43961 - t43963 - t43966 + t43973 - t43979 + t43989 - t43994;
    (t44080, t44082, t44085, t44089, t44092, t44115)
}
