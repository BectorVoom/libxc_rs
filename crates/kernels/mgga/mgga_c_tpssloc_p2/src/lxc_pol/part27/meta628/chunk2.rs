//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2115/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2115<F: Float>(t1877: F, t22959: F, t22961: F, t25013: F, t25015: F, t2522: F, t25366: F, t25372: F, t25375: F, t25385: F, t6666: F, t6670: F, t81483: F, t86703: F, t86707: F, t86710: F, t86714: F, t86718: F, t86722: F, t86727: F, t86734: F, t86736: F, t86740: F, t86746: F, t86751: F) -> F {
    let t86752 = -t1877 * t6670 * t86746 + F::new(3.0) * t2522 * t25385 * t6666 - F::new(3.0) * t22959 * t86710 - F::new(3.0) * t22959 * t86722 - F::new(3.0) * t22959 * t86727 - F::new(3.0) * t22961 * t86736 - F::new(3.0) * t25013 * t86707 + F::new(6.0) * t25015 * t86740 - F::new(3.0) * t25366 * t81483 + t25372 * t86714 - F::new(3.0) * t25372 * t86718 + F::new(2.0) * t25375 * t86703 - t86734 - t86751;
    t86752
}
