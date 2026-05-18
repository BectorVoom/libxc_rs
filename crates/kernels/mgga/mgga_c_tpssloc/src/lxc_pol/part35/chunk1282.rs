//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1282/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1282<F: Float>(t2627: F, t7510: F, t23030: F, t25258: F, t7524: F, t81612: F, t81613: F, t23145: F, t4166: F, t25132: F, t81876: F, t23047: F) -> (F, F, F, F, F, F) {
    let t87142 = t2627 * t7510;
    let t87155 = t23030 * t25258;
    let t87177 = t81612 * t81613 * t7524;
    let t87199 = t4166 * t23145;
    let t87213 = t81876 * t25132;
    let t87218 = t4166 * t23047;
    (t87142, t87155, t87177, t87199, t87213, t87218)
}
