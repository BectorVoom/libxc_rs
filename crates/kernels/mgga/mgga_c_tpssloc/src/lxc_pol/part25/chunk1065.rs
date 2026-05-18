//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1065/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1065<F: Float>(t13487: F, t1877: F, t193: F, t202: F, t2057: F, t2379: F, t24334: F, t24339: F, t24344: F, t2522: F, t2553: F, t2745: F, t2749: F, t4314: F, t7110: F, t7114: F, t776: F, t868: F, t870: F) -> F {
    let t24379 = t193 * t202 * t24334 * t870 - F::new(6.0) * t13487 * t2522 * t7114 - F::new(2.0) * t1877 * t24339 * t868 + F::new(2.0) * t1877 * t24344 * t2749 - t1877 * t2745 * t7114 + F::new(6.0) * t2057 * t2379 * t4314 + F::new(3.0) * t2057 * t2522 * t2553 + F::new(6.0) * t2522 * t7110 * t776;
    t24379
}
