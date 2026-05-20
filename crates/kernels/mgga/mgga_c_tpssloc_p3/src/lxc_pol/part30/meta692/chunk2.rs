//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2206/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2206<F: Float>(t1877: F, t1915: F, t22959: F, t23290: F, t25013: F, t2522: F, t25372: F, t28249: F, t28448: F, t28459: F, t5397: F, t606: F, t6666: F, t6670: F, t6671: F, t81483: F, t98046: F, t98050: F, t98054: F, t98059: F, t98065: F, t98071: F, t98075: F, t98079: F, t98082: F, t98086: F) -> F {
    let t98090 = -F::new(3.0) * t81483 * t28249 + t1877 * t28448 * t606 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2522 * t1915 * t98046 + F::new(3.0) / F::new(2.0) * t2522 * t1915 * t98050 - t1877 * t98054 * t6671 / F::new(2.0) - F::new(6.0) * t25013 * t98059 - t1877 * t23290 * t28459 + F::new(2.0) * t25372 * t98065 - t98071 + t1877 * t6666 * t5397 / F::new(2.0) - t1877 * t6670 * t98075 / F::new(2.0) - F::new(3.0) * t22959 * t98079 - t1877 * t6670 * t98082 / F::new(2.0) - t1877 * t6670 * t98086 / F::new(2.0);
    t98090
}
