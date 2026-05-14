//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 982/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk982<F: Float>(t31233: F, t31235: F, t31237: F, t31239: F, t31883: F, t31885: F, t31887: F, t32595: F, t32609: F, t671: F, t8446: F, t1393: F, t31055: F, t31057: F, t31060: F, t31077: F, t31088: F, t31089: F, t31223: F, t31249: F, t31898: F, t31900: F, t31902: F, t31904: F, t31906: F, t31909: F, t31916: F, t31919: F, t574: F, t672: F, t8916: F) -> (F, F) {
    let t32623 = 2.0 * t32609 * t671 + t31233 + t31235 + t31237 + t31239 + 4.0 * t31883 + 4.0 * t31885 + 4.0 * t31887 + t32595 + t8446;
    let t32628 = t1393 * t8916 - 2.0 * t32609 * t672 + t32623 * t574 - t31055 - t31057 - t31060 - t31077 - t31088 + t31089 + t31223 - t31249 - 4.0 * t31898 - 4.0 * t31900 - 4.0 * t31902 - 4.0 * t31904 - 4.0 * t31906 - 4.0 * t31909 + 6.0 * t31916 - 4.0 * t31919;
    (t32623, t32628)
}
