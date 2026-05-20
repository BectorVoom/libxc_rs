//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2132/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2132<F: Float>(t28030: F, t6535: F, t26114: F, t7461: F, t19994: F, t24995: F, t8945: F, t28831: F, t83886: F, t6287: F, t652: F, t6534: F) -> (F, F, F, F, F) {
    let t96738 = F::new(2.0) * t28030 * t6535;
    let t96740 = F::new(4.0) * t26114 * t7461;
    let t96746 = F::new(6.0) * t24995 * t8945 * t19994;
    let t96755 = F::new(6.0) * t83886 * t28831;
    let t96758 = F::new(2.0) * t652 * t6287 * t6534;
    (t96738, t96740, t96746, t96755, t96758)
}
