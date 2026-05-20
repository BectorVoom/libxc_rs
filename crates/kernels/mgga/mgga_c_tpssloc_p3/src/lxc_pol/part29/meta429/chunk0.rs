//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1719/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1719<F: Float>(t2020: F, t22607: F, t2314: F, t6535: F, t12823: F, t1874: F, t4034: F, t6525: F, t12734: F, t2006: F, t3752: F, t1323: F, t6955: F) -> (F, F, F, F, F, F, F, F) {
    let t22608 = t22607 * t2020;
    let t22610 = F::new(4.0) * t2314 * t6535;
    let t22612 = F::new(2.0) * t12823 * t1874;
    let t22614 = F::new(4.0) * t4034 * t6525;
    let t22616 = F::new(4.0) * t12734 * t1874;
    let t22618 = F::new(4.0) * t2314 * t6525;
    let t22622 = t3752 * t2006;
    let t22624 = t1323 * t6955;
    (t22608, t22610, t22612, t22614, t22616, t22618, t22622, t22624)
}
