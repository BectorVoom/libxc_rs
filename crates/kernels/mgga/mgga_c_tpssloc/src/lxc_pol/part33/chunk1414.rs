//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1414/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1414<F: Float>(t100945: F, t100946: F, t100949: F, t100952: F, t100960: F, t107545: F, t107588: F, t1398: F, t1852: F, t1858: F, t2023: F, t2029: F, t22431: F, t22453: F, t28869: F, t28904: F, t3: F, t580: F, t6471: F, t6483: F, t7759: F, t7774: F, t96348: F) -> F {
    let tv4rho3sigma9 = t107545 * t3 * t580 + t107588 * t1398 + F::new(3.0) * t1852 * t28904 + F::new(3.0) * t1858 * t28869 + t2023 * t22453 + t2029 * t22431 + F::new(3.0) * t6471 * t7774 + F::new(3.0) * t6483 * t7759 + F::new(3.0) * t100945 + F::new(3.0) * t100946 + F::new(6.0) * t100949 + F::new(3.0) * t100952 + F::new(6.0) * t100960 + F::new(3.0) * t96348;
    tv4rho3sigma9
}
