//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1339/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1339<F: Float>(t1675: F, t1860: F, t1861: F, t19411: F, t20713: F, t21123: F, t21136: F, t21165: F, t22151: F, t5483: F, t5975: F, t5976: F, t5979: F, t6077: F, t6475: F, t65169: F, t65172: F, t68115: F, t69087: F, t69338: F) -> (F,) {
    let t72892 = 2.0 / 3.0 * t21123 * t5976 + 2.0 / 3.0 * t21123 * t5979 + 5.0 / 3.0 * t68115 * t6077 - t5483 * t22151 / 6.0 - t1675 * t5975 * t21165 / 6.0 - t1675 * t1860 * t69338 / 6.0 + t69087 * t1861 / 3.0 + t21136 * t5976 / 3.0 + t21136 * t5979 / 3.0 - 10.0 / 3.0 * t65169 * t20713 - 10.0 / 3.0 * t65172 * t20713 + 2.0 / 3.0 * t19411 * t6475;
    (t72892,)
}
