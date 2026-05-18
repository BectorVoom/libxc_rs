//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1283/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1283<F: Float>(t32663: F, t4034: F, t1873: F, t25958: F, t652: F, t1874: F, t96361: F, t24999: F, t6525: F, t12725: F, t8323: F, t120714: F, t120716: F, t120719: F, t120721: F, t120723: F, t120728: F, t120730: F, t120732: F, t120735: F, t120738: F, t120740: F, t120742: F, t31062: F, t31224: F, t4028: F, t4077: F) -> F {
    let t120744 = t4034 * t32663;
    let t120747 = t652 * t25958 * t1873;
    let t120749 = t96361 * t1874;
    let t120751 = t24999 * t6525;
    let t120753 = t12725 * t8323;
    let t120755 = -F::new(2.0) * t31062 * t4028 - F::new(2.0) * t31224 * t4077 - F::new(4.0) * t120714 - F::new(4.0) * t120716 - t120719 - t120721 - F::new(4.0) * t120723 - t120728 - t120730 - F::new(4.0) * t120732 - t120735 - F::new(2.0) * t120738 - F::new(4.0) * t120740 - F::new(4.0) * t120742 - F::new(4.0) * t120744 - F::new(4.0) * t120747 - F::new(4.0) * t120749 - F::new(4.0) * t120751 - F::new(4.0) * t120753;
    t120755
}
