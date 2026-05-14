//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 736/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk736<F: Float>(t1873: F, t2363: F, t3941: F, t1401: F, t22479: F, t2319: F, t23862: F, t23877: F, t23880: F, t23886: F, t23888: F, t23890: F, t23892: F, t23895: F, t577: F, t671: F, t7010: F) -> (F, F) {
    let t23896 = t1873 * t2363;
    let t23898 = 27.0 * t3941 * t23896;
    let t23900 = 0.135e2 * t1401 * t22479;
    let t23901 = 0.45e1 * t23862 * t577 + 27.0 * t23877 * t671 + 27.0 * t23880 * t2319 + 0.135e2 * t7010 * t2363 + t23886 + t23888 + t23890 + t23892 + t23895 + t23898 + t23900;
    (t23896, t23901)
}
