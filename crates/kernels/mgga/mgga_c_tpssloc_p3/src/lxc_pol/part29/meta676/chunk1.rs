//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2267/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2267<F: Float>(t26179: F, t6535: F, t2314: F, t25994: F, t12823: F, t7461: F, t25980: F, t4034: F, t12813: F, t89: F, t1874: F, t6525: F) -> (F, F, F, F, F, F) {
    let t91739 = F::new(4.0) * t26179 * t6535;
    let t91747 = F::new(4.0) * t2314 * t25994;
    let t91749 = F::new(2.0) * t12823 * t7461;
    let t91752 = F::new(4.0) * t4034 * t25980;
    let t91753 = t89 * t12813;
    let t91755 = F::new(2.0) * t91753 * t1874;
    let t91757 = F::new(4.0) * t26179 * t6525;
    (t91739, t91747, t91749, t91752, t91755, t91757)
}
