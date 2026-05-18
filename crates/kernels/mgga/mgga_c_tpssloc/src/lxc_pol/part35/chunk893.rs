//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 893/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk893<F: Float>(t1174: F, t15363: F, t1420: F, t1887: F, t337: F, t1714: F, t4899: F, t15026: F, t3032: F, t3514: F, t1742: F, t3036: F) -> (F, F, F, F, F, F) {
    let t15364 = t1174 * t15363;
    let t15376 = t1420 * t337 * t1887;
    let t15390 = t4899 * t1714;
    let t15437 = t15026 * t3032;
    let t15438 = t15437 * t3514;
    let t15501 = t1742 * t3036;
    (t15364, t15376, t15390, t15437, t15438, t15501)
}
