//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2365/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2365<F: Float>(t25980: F, t4034: F, t12813: F, t89: F, t1874: F, t26179: F, t6525: F, t22561: F, t7458: F, t3652: F, t652: F, t7467: F) -> (F, F, F, F, F) {
    let t91752 = F::new(4.0) * t4034 * t25980;
    let t91753 = t89 * t12813;
    let t91755 = F::new(2.0) * t91753 * t1874;
    let t91757 = F::new(4.0) * t26179 * t6525;
    let t91759 = F::new(4.0) * t7458 * t22561;
    let t91762 = F::new(2.0) * t652 * t3652 * t7467;
    (t91752, t91755, t91757, t91759, t91762)
}
