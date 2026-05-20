//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1044/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1044<F: Float>(t1877: F, t25: F, t2522: F, t30753: F, t30757: F, t30767: F, t30770: F, t606: F, t6542: F, t6670: F, t6671: F, t8366: F, t8370: F) -> F {
    let t30776 = F::new(3.0) / F::new(2.0) * t2522 * t8366 * t6542 + t1877 * t30753 * t25 / F::new(2.0) - t1877 * t30757 * t6671 / F::new(2.0) + t1877 * t8366 * t606 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t2522 * t8370 * t6542 - t1877 * t6670 * t30767 + t1877 * t30770 * t6671 - t1877 * t8370 * t606 / F::new(2.0);
    t30776
}
