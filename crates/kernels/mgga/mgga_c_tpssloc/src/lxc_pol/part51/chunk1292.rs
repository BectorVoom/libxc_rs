//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1292/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1292<F: Float>(t122645: F, t122656: F, t122659: F, t122662: F, t122664: F, t122667: F, t122671: F, t1393: F, t1976: F, t22461: F, t26103: F, t26880: F, t26967: F, t27163: F, t33085: F, t33601: F, t6517: F, t7057: F, t7796: F, t8450: F) -> (F,) {
    let t122673 = t1393 * t33601 - t1976 * t26967 - 2.0 * t22461 * t7796 - 2.0 * t26103 * t7796 - t26880 * t8450 - 2.0 * t27163 * t6517 - 2.0 * t33085 * t7057 - t122645 + t122656 - t122659 - t122662 - t122664 + t122667 + t122671;
    (t122673,)
}
