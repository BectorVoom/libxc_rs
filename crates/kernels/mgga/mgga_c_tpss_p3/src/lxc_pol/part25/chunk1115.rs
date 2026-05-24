//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1115/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1115<F: Float>(t1027: F, t15303: F, t5085: F, t9267: F, t4071: F, t4079: F, t2868: F, t5092: F, t15248: F, t15251: F, t15292: F, t15294: F, t15296: F, t15299: F, t15301: F) -> (F, F, F, F, F) {
    let t15304 = t15303 * t1027;
    let t15306 = t9267 * t5085;
    let t15307 = t15306 * t1027;
    let t15309 = t4071 * t4079;
    let t15311 = t2868 * t5092;
    let t15312 = t15311 * t1027;
    let t15314 = -F::new(0.5519e-1) * t15248 + F::new(0.301925e0) * t15251 + F::new(0.258925e1) * t15292 + F::new(0.16504875e0) * t15294 + F::cast_from(0.18396666666666666667e-1_f64) * t15296 - F::cast_from(0.412621875e-1_f64) * t15299 + F::new(0.16504875e0) * t15301 + F::new(0.82524375e-1) * t15304 + F::new(0.19419375e1) * t15307 - F::new(0.258925e1) * t15309 - F::new(0.1294625e1) * t15312;
    (t15304, t15307, t15309, t15312, t15314)
}
