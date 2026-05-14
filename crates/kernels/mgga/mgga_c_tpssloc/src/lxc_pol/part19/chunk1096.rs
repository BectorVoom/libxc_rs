//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1096/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1096<F: Float>(t120: F, t12167: F, t12331: F, t1358: F, t12250: F, t3850: F, t10021: F, t154: F, t59: F, t3749: F, t598: F, t535: F, t795: F, t215: F, t39933: F, t12227: F, t9577: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40304 = t120 * t12167;
    let t40329 = t12331 * t1358;
    let t40335 = t12250 * t3850;
    let t40341 = t59 * t10021 * t154;
    let t40343 = 0.99537037037037037035e-1 * t40341 * t3749;
    let t40344 = t59 * t598;
    let t40347 = 0.11265432098765432099e0 * t40344 * t535 * t795;
    let t40350 = 0.14979423868312757201e0 * t39933 * t535 * t215;
    let t40351 = t9577 * t12227;
    (t40304, t40329, t40335, t40341, t40343, t40344, t40347, t40350, t40351)
}
