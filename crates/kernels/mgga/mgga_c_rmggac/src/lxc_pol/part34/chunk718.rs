//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 718/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk718<F: Float>(t15252: F, t2144: F, t333: F, t3351: F, t7231: F, t498: F, t8946: F, t3352: F, t8947: F, t15128: F, t321: F, t262: F, t7204: F, t7192: F, t15098: F, t1326: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t74948 = t3351 * t7231 * t2144 * t15252 * t333;
    let t74953 = t3351 * t7231 * t2144 * t8946 * t498;
    let t74957 = t3351 * t3352 * t2144 * t8947;
    let t74959 = t15128 * t321;
    let t74960 = t262 * t74959;
    let t74961 = t7204 * t74960;
    let t74963 = t15128 * t333;
    let t74964 = t262 * t74963;
    let t74965 = t7192 * t74964;
    let t74967 = t15098 * t321;
    let t74968 = t1326 * t74967;
    (t74948, t74953, t74957, t74959, t74960, t74961, t74963, t74964, t74965, t74967, t74968)
}
