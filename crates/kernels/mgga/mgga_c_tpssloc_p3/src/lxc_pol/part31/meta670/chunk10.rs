//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1999/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1999<F: Float>(t26959: F, t7428: F, t27979: F, t7032: F, t1860: F, t27956: F, t7031: F, t91890: F, t91894: F, t91896: F, t91898: F, t91900: F, t91904: F, t91905: F, t91913: F, t91921: F) -> F {
    let t102137 = t7428 * t26959;
    let t102139 = t27979 * t7032;
    let t102142 = t1860 * t7031 * t27956;
    let t102145 = -F::new(16.0) / F::new(9.0) * t102137 + F::new(16.0) / F::new(9.0) * t102139 - F::new(8.0) / F::new(9.0) * t102142 + t91890 + t91894 + t91896 + t91898 + t91900 + t91904 - F::new(352.0) / F::new(27.0) * t91905 + t91913 + t91921;
    t102145
}
