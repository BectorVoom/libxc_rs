//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1997/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1997<F: Float>(t12971: F, t13471: F, t13487: F, t16596: F, t1877: F, t193: F, t202: F, t2057: F, t24191: F, t24339: F, t24344: F, t2522: F, t25365: F, t26563: F, t26740: F, t26744: F, t4119: F, t4255: F, t4303: F, t4314: F, t47645: F, t57912: F, t57921: F, t59580: F, t7110: F, t7114: F, t776: F, t7856: F, t86706: F, t870: F, t89733: F, t92989: F) -> F {
    let t93052 = -t1877 * t7114 * t13471 + F::new(3.0) * t2522 * t2057 * t12971 + F::new(12.0) * t24191 * t89733 - F::new(12.0) * t26563 * t57912 - F::new(6.0) * t2522 * t26744 * t13487 + F::new(6.0) * t47645 * t7856 + F::new(6.0) * t2522 * t26740 * t776 - F::new(6.0) * t4314 * t7114 * t86706 + F::new(6.0) * t2522 * t7110 * t4119 + t193 * t202 * t92989 * t870 + F::new(6.0) * t2522 * t24344 * t57921 - F::new(6.0) * t2522 * t24339 * t16596 - F::new(6.0) * t2522 * t24339 * t25365 - F::new(3.0) * t2522 * t7114 * t59580 - F::new(2.0) * t1877 * t24339 * t4303 + F::new(12.0) * t4314 * t7110 * t4255;
    t93052
}
