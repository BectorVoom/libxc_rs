//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1192/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1192<F: Float>(t6534: F, t9348: F, t1268: F, t81455: F, t22479: F, t2314: F, t1873: F, t45814: F, t12739: F, t5113: F, t1401: F, t12521: F, t3938: F, t3941: F, t9416: F, t16535: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t83958 = 6.0 * t9348 * t6534;
    let t83960 = 2.0 * t1268 * t81455;
    let t83962 = 6.0 * t2314 * t22479;
    let t83964 = 2.0 * t45814 * t1873;
    let t83966 = 6.0 * t12739 * t6534;
    let t83968 = 6.0 * t5113 * t22479;
    let t83979 = 0.135e2 * t1401 * t81455;
    let t83984 = 0.405e2 * t12521 * t6534;
    let t83988 = 0.405e2 * t3938 * t22479;
    let t83991 = 27.0 * t3941 * t1873 * t9416;
    let t83993 = 81.0 * t16535 * t6534;
    (t83958, t83960, t83962, t83964, t83966, t83968, t83979, t83984, t83988, t83991, t83993)
}
