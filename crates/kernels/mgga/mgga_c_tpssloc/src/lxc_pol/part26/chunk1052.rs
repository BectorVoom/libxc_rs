//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1052/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1052<F: Float>(t2019: F, t23857: F, t1983: F, t12521: F, t1873: F, t12524: F, t7015: F, t3938: F, t6534: F, t16535: F, t671: F, t3941: F, t2363: F, t1401: F, t22479: F, t2274: F, t50: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23858 = t2019 * t23857;
    let t23860 = 2.0 * t1983 * t23858;
    let t23886 = 0.135e2 * t12521 * t1873;
    let t23888 = 54.0 * t12524 * t7015;
    let t23890 = 27.0 * t3938 * t6534;
    let t23892 = 27.0 * t16535 * t1873;
    let t23893 = t6534 * t671;
    let t23895 = 54.0 * t3941 * t23893;
    let t23896 = t1873 * t2363;
    let t23898 = 27.0 * t3941 * t23896;
    let t23900 = 0.135e2 * t1401 * t22479;
    let t24498 = t50 * t2274;
    (t23858, t23860, t23886, t23888, t23890, t23892, t23893, t23895, t23896, t23898, t23900, t24498)
}
