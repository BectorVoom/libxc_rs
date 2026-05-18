//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 966/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk966<F: Float>(t135: F, t4930: F, t1174: F, t1420: F, t1887: F, t337: F, t11570: F, t3961: F, t1714: F, t4899: F, t11545: F, t60: F) -> (F, F, F, F, F) {
    let t15372 = t135 * t4930;
    let t15374 = F::new(0.55555555555555555554e-3) * t1174 * t15372;
    let t15376 = t1420 * t337 * t1887;
    let t15382 = t11570 * t3961;
    let t15390 = t4899 * t1714;
    let t15394 = t60 * t11545;
    (t15374, t15376, t15382, t15390, t15394)
}
