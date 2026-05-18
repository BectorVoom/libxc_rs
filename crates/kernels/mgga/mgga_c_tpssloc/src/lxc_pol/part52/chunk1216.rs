//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1216/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1216<F: Float>(t28: F, t7540: F, t1649: F, t1877: F, t2522: F, t30757: F, t30770: F, t32886: F, t6670: F, t7649: F, t7656: F, t8366: F, t8370: F) -> (F, F) {
    let t33065 = t28 * t7540;
    let t33073 = F::new(3.0) / F::new(2.0) * t2522 * t8366 * t7649 + t1877 * t32886 * t28 / F::new(2.0) - t1877 * t30757 * t7656 / F::new(2.0) + t1877 * t8366 * t1649 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t2522 * t8370 * t7649 - t1877 * t6670 * t33065 + t1877 * t30770 * t7656 - t1877 * t8370 * t1649 / F::new(2.0);
    (t33065, t33073)
}
