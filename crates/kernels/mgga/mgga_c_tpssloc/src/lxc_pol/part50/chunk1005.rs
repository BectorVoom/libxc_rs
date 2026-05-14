//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1005/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1005<F: Float>(t33049: F, t33079: F, t15899: F, t8493: F, t1983: F, t1458: F, t1868: F) -> (F, F, F, F) {
    let t33080 = t33049 + t33079;
    let t33082 = t8493 * t15899;
    let t33084 = 2.0 * t1983 * t33082;
    let t33085 = t1868 * t1458;
    (t33080, t33082, t33084, t33085)
}
