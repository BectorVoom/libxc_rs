//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2269/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2269<F: Float>(t25989: F, t83886: F, t25994: F, t4034: F, t15857: F, t1873: F, t652: F, t1874: F, t45632: F, t26135: F, t3941: F, t671: F) -> (F, F, F, F, F) {
    let t91771 = F::new(6.0) * t83886 * t25989;
    let t91777 = F::new(4.0) * t4034 * t25994;
    let t91780 = F::new(2.0) * t652 * t15857 * t1873;
    let t91782 = F::new(2.0) * t45632 * t1874;
    let t91799 = F::new(54.0) * t3941 * t26135 * t671;
    (t91771, t91777, t91780, t91782, t91799)
}
