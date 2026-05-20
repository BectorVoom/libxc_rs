//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1956/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1956<F: Float>(t26055: F, t7032: F, t22531: F, t22537: F, t23963: F, t26911: F, t6492: F, t7782: F, t90196: F, t91890: F, t91894: F, t91896: F, t91898: F, t91900: F, t91904: F, t91905: F, t91907: F) -> F {
    let t91913 = F::new(32.0) / F::new(9.0) * t26055 * t7032;
    let t91914 = t91890 - F::new(2.0) / F::new(3.0) * t22537 * t7782 + t91894 + t91896 + t91898 + t91900 + F::new(10.0) * t23963 * t90196 + t91904 - F::new(176.0) / F::new(27.0) * t91905 - F::new(10.0) / F::new(3.0) * t91907 * t6492 - F::new(5.0) / F::new(3.0) * t26911 * t22531 + t91913;
    t91914
}
