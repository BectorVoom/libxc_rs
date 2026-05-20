//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1425/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1425<F: Float>(t5107: F, t671: F, t1266: F, t4072: F, t1774: F, t2363: F, t584: F, t9212: F, t9214: F, t9216: F, t9218: F, t9220: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12545 = t5107 * t671;
    let t12550 = t1266 * t4072;
    let t12557 = t1774 * t2363;
    let t12560 = F::new(0.348e1) * t584;
    let t12561 = F::new(0.156e1) * t9212;
    let t12562 = F::new(0.312e1) * t9214;
    let t12563 = F::new(0.2312e3) * t9216;
    let t12564 = F::new(0.3468e3) * t9218;
    let t12565 = F::new(0.56952e3) * t9220;
    (t12545, t12550, t12557, t12560, t12561, t12562, t12563, t12564, t12565)
}
