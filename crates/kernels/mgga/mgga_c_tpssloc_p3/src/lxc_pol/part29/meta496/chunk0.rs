//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1852/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1852<F: Float>(t5308: F, t8945: F, t24995: F, t1874: F, t19456: F, t4028: F, t6525: F, t5161: F, t6996: F, t1983: F, t1914: F, t193: F, t200: F) -> (F, F, F, F, F, F, F) {
    let t24996 = t8945 * t5308;
    let t24998 = F::new(6.0) * t24995 * t24996;
    let t25005 = F::new(2.0) * t19456 * t1874;
    let t25007 = F::new(2.0) * t4028 * t6525;
    let t25010 = t6996 * t5161;
    let t25011 = t1983 * t25010;
    let t25013 = t193 * t200 * t1914;
    (t24996, t24998, t25005, t25007, t25010, t25011, t25013)
}
