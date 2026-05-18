//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1042/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1042<F: Float>(t113836: F, t1862: F, t8513: F, t39054: F, t8511: F, t39063: F, t2241: F, t8514: F, t31687: F, t9239: F, t31677: F, t2244: F) -> (F, F, F, F, F, F) {
    let t115863 = t8513 * t113836 * t1862;
    let t115866 = t39054 * t8511;
    let t115871 = t39063 * t8511;
    let t115873 = t8513 * t8514 * t2241;
    let t115876 = t9239 * t31687;
    let t115877 = t115876 * t31677;
    let t115880 = t8513 * t8514 * t2244;
    (t115863, t115866, t115871, t115873, t115877, t115880)
}
