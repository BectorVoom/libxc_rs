//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1211/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1211<F: Float>(t119733: F, t119783: F, t31222: F, t7685: F, t24987: F, t8494: F, t26142: F, t8526: F, t2314: F, t32677: F, t4034: F, t5107: F, t652: F, t8326: F) -> (F, F, F, F, F, F, F) {
    let t119784 = t119733 + t119783;
    let t119795 = t7685 * t31222;
    let t119796 = t24987 * t8494;
    let t119810 = F::new(4.0) * t8526 * t26142;
    let t119824 = F::new(2.0) * t2314 * t32677;
    let t119826 = F::new(2.0) * t4034 * t32677;
    let t119830 = F::new(2.0) * t652 * t5107 * t8326;
    (t119784, t119795, t119796, t119810, t119824, t119826, t119830)
}
