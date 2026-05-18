//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 688/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk688<F: Float>(t5264: F, t5266: F, t2408: F, t2417: F, t2426: F, t2486: F, t3688: F, t3813: F, t6299: F, t6304: F, t6329: F, t2423: F, t3686: F, t3690: F, t3695: F, t3819: F, t3821: F, t3823: F, t3825: F, t3832: F, t3836: F, t6300: F, t6322: F) -> (F, F, F, F) {
    let t6399 = F::new(8.0) * t5264;
    let t6400 = F::new(8.0) * t5266;
    let t6401 = t6329 + t6304 + t3813 - t2486 - t6299 + t2408 + t2417 - t6399 - t6400 - t2426 + t3688;
    let t6402 = -t3690 - t3695 + t6322 + t3686 + t3819 + t3821 + t3823 - t2423 - t6300 + t3825 - t3832 - t3836;
    (t6399, t6400, t6401, t6402)
}
