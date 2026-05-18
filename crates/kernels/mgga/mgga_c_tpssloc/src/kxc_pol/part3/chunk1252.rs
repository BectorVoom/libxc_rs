//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1252/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1252<F: Float>(t12832: F, t16505: F, t3: F, t1395: F, t1858: F, t5381: F, t576: F, t112: F, t5363: F, t111: F, t1851: F, t2319: F) -> (F, F, F, F, F, F, F) {
    let t16506 = t12832 + t16505;
    let t16507 = t3 * t16506;
    let t16513 = F::new(2.0) * t1395 * t1858;
    let t16515 = F::new(2.0) * t576 * t5381;
    let t16521 = t5363 * t112;
    let t16524 = t1851 * t111;
    let t16535 = t576 * t2319;
    (t16506, t16507, t16513, t16515, t16521, t16524, t16535)
}
