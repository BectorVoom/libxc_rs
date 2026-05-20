//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2001/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2001<F: Float>(t1081: F, t1649: F, t1877: F, t2057: F, t23789: F, t23813: F, t24191: F, t24335: F, t2522: F, t26563: F, t26740: F, t26744: F, t26756: F, t3231: F, t4314: F, t47645: F, t7114: F, t7649: F, t7845: F, t7871: F, t89859: F, t89862: F, t89865: F, t89868: F, t89874: F, t89896: F, t89904: F, t89954: F, t92319: F) -> F {
    let t93181 = F::new(3.0) / F::new(2.0) * t2522 * t24335 * t7649 - t1877 * t26744 * t23813 / F::new(2.0) + F::new(3.0) * t47645 * t7871 - F::new(3.0) * t26563 * t89865 - F::new(3.0) * t26756 * t89954 + t1877 * t26740 * t1081 + F::new(6.0) * t26563 * t89896 + t1877 * t24335 * t1649 / F::new(2.0) - F::new(3.0) * t24191 * t89862 - F::new(3.0) * t92319 * t23789 + F::new(6.0) * t26563 * t89859 - t1877 * t7114 * t89868 / F::new(2.0) + t1877 * t7845 * t3231 / F::new(2.0) + F::new(3.0) * t4314 * t2057 * t89874 + F::new(3.0) * t24191 * t89904;
    t93181
}
