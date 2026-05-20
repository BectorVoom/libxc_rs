//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2268/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2268<F: Float>(t22561: F, t7458: F, t3652: F, t652: F, t7467: F, t22579: F, t7685: F, t1874: F, t55934: F, t12725: F, t6525: F, t26168: F, t6876: F) -> (F, F, F, F, F, F) {
    let t91759 = F::new(4.0) * t7458 * t22561;
    let t91762 = F::new(2.0) * t652 * t3652 * t7467;
    let t91763 = t7685 * t22579;
    let t91765 = F::new(4.0) * t55934 * t1874;
    let t91767 = F::new(4.0) * t12725 * t6525;
    let t91769 = F::new(6.0) * t6876 * t26168;
    (t91759, t91762, t91763, t91765, t91767, t91769)
}
