//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1032/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1032<F: Float>(t1874: F, t91857: F, t26977: F, t6525: F, t22585: F, t8607: F, t31304: F, t7000: F, t112620: F, t112621: F, t112622: F, t115690: F, t115695: F, t115698: F, t115700: F, t115702: F, t115704: F, t22619: F, t2323: F, t23938: F, t23953: F, t31246: F, t31532: F, t6539: F, t7042: F, t7220: F, t8450: F) -> F {
    let t115708 = F::new(2.0) * t91857 * t1874;
    let t115712 = F::new(4.0) * t26977 * t6525;
    let t115716 = F::new(3.0) * t8607 * t22585;
    let t115718 = F::new(2.0) * t31304 * t7000;
    let t115719 = -F::new(4.0) * t22619 * t7042 - F::new(4.0) * t2323 * t31532 - F::new(4.0) * t23938 * t6539 + F::new(3.0) * t23953 * t8450 - F::new(2.0) * t31246 * t7220 - t112620 - t112621 - t112622 + t115690 + t115695 - t115698 + t115700 - t115702 - t115704 - t115708 - t115712 + t115716 - t115718;
    t115719
}
