//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 903/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk903<F: Float>(t4733: F, t7286: F, t7285: F, t1716: F, t24638: F, t1760: F, t7391: F, t3598: F, t24574: F, t8003: F, t7295: F, t6686: F, t8020: F) -> (F, F, F, F, F, F, F, F) {
    let t27388 = t7286 * t4733;
    let t27389 = t7285 * t27388;
    let t27392 = t1716 * t24638;
    let t27395 = t7391 * t1760;
    let t27396 = t3598 * t27395;
    let t27401 = t24574 * t8003;
    let t27403 = t1716 * t7295;
    let t27406 = t8020 * t6686;
    (t27388, t27389, t27392, t27395, t27396, t27401, t27403, t27406)
}
