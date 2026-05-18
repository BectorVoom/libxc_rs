//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1340/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1340<F: Float>(t105780: F, t105787: F, t105797: F, t105801: F, t105810: F, t105814: F, t105818: F, t105822: F, t1408: F, t1877: F, t1915: F, t23295: F, t25013: F, t2522: F, t25358: F, t28252: F, t28256: F, t28448: F, t28456: F, t28459: F, t28462: F, t4314: F, t6670: F, t7475: F, t7541: F, t82312: F, t87975: F) -> F {
    let t105829 = -F::new(3.0) / F::new(2.0) * t1877 * t25358 * t28462 - F::new(3.0) / F::new(2.0) * t1877 * t6670 * t105780 + F::new(9.0) / F::new(2.0) * t2522 * t28448 * t7475 - t1877 * t6670 * t105787 / F::new(2.0) + F::new(3.0) * t1877 * t87975 * t28456 + F::new(9.0) * t2522 * t7541 * t28252 + F::new(3.0) / F::new(2.0) * t2522 * t1915 * t105797 + F::new(9.0) * t25013 * t105801 - F::new(3.0) * t1877 * t25358 * t28459 + F::new(9.0) / F::new(2.0) * t2522 * t7541 * t28256 + F::new(9.0) * t4314 * t1915 * t105810 + F::new(3.0) * t1877 * t23295 * t105814 - F::new(3.0) / F::new(2.0) * t1877 * t6670 * t105818 - F::new(3.0) * t1877 * t82312 * t105822 + F::new(3.0) / F::new(2.0) * t1877 * t28448 * t1408;
    t105829
}
