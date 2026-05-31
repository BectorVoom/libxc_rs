//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1710/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1710<F: Float>(t111: F, t1395: F, t5107: F, t671: F, t1266: F, t4072: F, t1774: F, t2363: F, t584: F, t9212: F, t9214: F, t9216: F) -> (F, F, F, F, F, F, F, F) {
    let t12524 = t1395 * t111;
    let t12545 = t5107 * t671;
    let t12550 = t1266 * t4072;
    let t12557 = t1774 * t2363;
    let t12560 = F::cast_from(0.348e1_f64) * t584;
    let t12561 = F::cast_from(0.156e1_f64) * t9212;
    let t12562 = F::cast_from(0.312e1_f64) * t9214;
    let t12563 = F::cast_from(0.2312e3_f64) * t9216;
    (t12524, t12545, t12550, t12557, t12560, t12561, t12562, t12563)
}
