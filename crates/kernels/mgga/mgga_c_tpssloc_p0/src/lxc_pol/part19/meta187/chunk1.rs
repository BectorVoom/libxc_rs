//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 842/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk842<F: Float>(t10003: F, t2618: F, t2623: F, t2630: F, t2635: F, t2643: F, t2681: F, t2703: F, t843: F, t849: F, t9967: F, t9974: F, t9978: F, t9983: F, t9986: F, t9988: F, t9990: F, t9994: F, t9997: F) -> F {
    let t10006 = -t2618 * t2681 / F::new(1024.0) + t9967 * t2635 / F::new(512.0) - t9974 * t9978 / F::new(512.0) + t2630 * t9983 / F::new(512.0) + F::new(7.0) / F::new(1536.0) * t9986 - F::new(35.0) / F::new(384.0) * t9988 - t9990 * t849 / F::new(256.0) + F::new(7.0) / F::new(192.0) * t9994 - t843 * t9997 / F::new(768.0) + F::new(5.0) / F::new(256.0) * t2623 * t2703 + t2643 * t10003 / F::new(256.0);
    t10006
}
