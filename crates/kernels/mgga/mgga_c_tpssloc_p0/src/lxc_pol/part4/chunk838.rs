//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 838/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk838<F: Float>(t1799: F, t550: F, t3805: F, t5249: F, t5264: F, t5266: F, t2408: F, t2417: F, t2426: F, t2486: F, t3688: F, t3813: F, t6299: F, t6304: F, t6329: F) -> (F, F, F, F, F) {
    let t6394 = t550 * t1799;
    let t6396 = t3805 * t5249 * t6394;
    let t6399 = F::new(8.0) * t5264;
    let t6400 = F::new(8.0) * t5266;
    let t6401 = t6329 + t6304 + t3813 - t2486 - t6299 + t2408 + t2417 - t6399 - t6400 - t2426 + t3688;
    (t6394, t6396, t6399, t6400, t6401)
}
