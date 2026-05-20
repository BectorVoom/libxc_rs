//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2000/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2000<F: Float>(t1877: F, t2057: F, t23792: F, t23807: F, t24191: F, t24339: F, t2522: F, t25892: F, t25898: F, t25928: F, t25938: F, t25945: F, t26563: F, t28: F, t7110: F, t7845: F, t84797: F, t89843: F, t89881: F, t89928: F, t89972: F, t89987: F, t92271: F, t92295: F, t92299: F, t92990: F, t93000: F) -> F {
    let t93144 = F::new(3.0) * t2522 * t7110 * t25938 - F::new(6.0) * t26563 * t89928 - t1877 * t24339 * t25945 + F::new(3.0) / F::new(2.0) * t2522 * t2057 * t89881 + F::new(3.0) * t26563 * t89843 + F::new(6.0) * t92295 * t25892 - F::new(3.0) * t24191 * t89987 - F::new(3.0) * t24191 * t89972 - F::new(3.0) * t84797 * t25898 + F::new(3.0) * t2522 * t7845 * t23792 - t92299 + t1877 * t92990 * t28 / F::new(2.0) + F::new(2.0) * t92271 * t25928 + t1877 * t93000 * t23807;
    t93144
}
