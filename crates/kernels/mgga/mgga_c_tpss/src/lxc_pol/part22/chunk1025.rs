//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1025/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1025<F: Float>(t10980: F, t10986: F, t11003: F, t11005: F, t11006: F, t11010: F, t11015: F, t11020: F, t11024: F, t11028: F, t11033: F, t11037: F, t8605: F, t8607: F, t8616: F, t8618: F, t8687: F) -> F {
    let t11040 = -t8687 - F::new(8.0) / F::new(27.0) * t8616 + F::new(2.0) / F::new(27.0) * t8607 - F::new(2.0) / F::new(9.0) * t8618 + t8605 / F::new(9.0) - F::new(4.0) / F::new(27.0) * t10980 + t11003 - t11005 + t11006 - F::new(10.0) / F::new(27.0) * t11010 + F::new(4.0) / F::new(3.0) * t11015 - F::new(4.0) / F::new(9.0) * t11020 - F::new(2.0) / F::new(9.0) * t11024 - F::new(2.0) * t11028 + F::new(4.0) / F::new(3.0) * t11033 + F::new(2.0) / F::new(3.0) * t11037 - t10986 / F::new(3.0);
    t11040
}
