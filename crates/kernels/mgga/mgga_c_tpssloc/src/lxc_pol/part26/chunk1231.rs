//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1231/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1231<F: Float>(t7288: F, t85660: F, t225: F, t24758: F, t24637: F, t7294: F, t2121: F, t3427: F, t7295: F, t11598: F, t1186: F, t11868: F, t11928: F, t11934: F, t11935: F, t1238: F, t1251: F, t1252: F, t2144: F, t2155: F, t24615: F, t24867: F, t24893: F, t3481: F, t3598: F, t3600: F, t44412: F, t462: F, t497: F, t498: F, t7283: F, t7300: F, t7348: F, t7351: F, t7392: F) -> (F,) {
    let t86473 = t85660 * t7288;
    let t86475 = t24758 * t225;
    let t86494 = t7294 * t24637;
    let t86501 = t2121 * t3427 * t7295;
    let t86506 = 0.49348022005446793095e-1 * t7283 * t7300 * t24615 * t11934 + 0.18277045187202515961e-2 * t86473 - 3.0 * t86475 * t1252 + t11598 * t2144 * t498 + 3.0 * t3481 * t7348 * t498 + 6.0 * t1238 * t3598 * t24867 * t1251 + 0.82246703342411321825e-2 * t2121 * t462 * t11868 * t225 * t497 + 6.0 * t24893 * t3600 + 0.49348022005446793095e-1 * t7283 * t1186 * t86494 + 6.0 * t7351 * t11935 - 0.54831135561607547884e-2 * t86501 - 3.0 * t11928 * t7392 - t44412 * t2155;
    (t86506,)
}
