//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1166/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1166<F: Float>(t1395: F, t8256: F, t1404: F, t8240: F, t2186: F, t5381: F, t30217: F, t580: F, t1858: F, t8153: F, t2193: F, t5363: F, t30263: F, t576: F, t6470: F, t1851: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110882 = 2.0 * t1395 * t8256;
    let t110884 = 2.0 * t8240 * t1404;
    let t110886 = 2.0 * t2186 * t5381;
    let t110888 = 2.0 * t30217 * t580;
    let t110899 = 2.0 * t8153 * t1858;
    let t110904 = 2.0 * t5363 * t2193;
    let t110910 = 2.0 * t576 * t30263;
    let t111316 = t6470 * t2193;
    let t111317 = t1851 * t8256;
    (t110882, t110884, t110886, t110888, t110899, t110904, t110910, t111316, t111317)
}
