//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 992/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk992<F: Float>(t121660: F, t126409: F, t126412: F, t126417: F, t126418: F, t126419: F, t126422: F, t126423: F, t2054: F, t25168: F, t26700: F, t26713: F, t26728: F, t2718: F, t28311: F, t28316: F, t31423: F, t33405: F, t5637: F, t5658: F, t7087: F, t7517: F, t7537: F, t7538: F, t7841: F, t855: F, t98166: F, t98279: F) -> F {
    let t127947 = F::new(4.0) * t26700 * t7517 + F::new(4.0) * t855 * t2718 * t7841 * t7537 - t31423 * t5658 + F::new(0.38381794893125283518e-1) * t121660 - t126409 - t126412 - t126417 + t126418 - t126419 + F::new(2.0) * t31423 * t5637 + t126422 - t98166 * t2054 - F::new(6.0) * t7087 * t28311 - F::new(2.0) * t26713 * t7538 - F::new(12.0) * t98279 * t33405 + t126423 - F::new(6.0) * t25168 * t26728 * t28316;
    t127947
}
