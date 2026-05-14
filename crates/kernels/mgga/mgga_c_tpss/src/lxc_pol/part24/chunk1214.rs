//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1214/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1214<F: Float>(t1678: F, t21165: F, t1675: F, t1680: F, t18345: F, t19346: F, t19349: F, t19393: F, t19425: F, t21116: F, t21123: F, t21129: F, t21133: F, t21136: F, t21139: F, t21146: F, t21159: F, t21162: F, t5487: F, t6073: F, t6077: F, t6080: F, t6087: F, t6091: F) -> (F, F) {
    let t21166 = t1678 * t21165;
    let t21169 = -5.0 * t18345 * t21116 - 10.0 / 3.0 * t19349 * t19346 + 5.0 / 3.0 * t19393 * t6077 + 2.0 / 3.0 * t21123 * t1680 + 5.0 / 3.0 * t19425 * t6077 + 5.0 / 3.0 * t5487 * t21129 + 5.0 / 6.0 * t5487 * t21133 + t21136 * t1680 / 3.0 + t21139 * t1680 / 3.0 + 2.0 / 3.0 * t6080 * t6087 + 2.0 / 3.0 * t6080 * t6091 - t21146 * t1680 / 6.0 - t6073 * t6087 / 3.0 - t6073 * t6091 / 3.0 - t1675 * t21159 / 6.0 - t1675 * t21162 / 3.0 - t1675 * t21166 / 6.0;
    (t21166, t21169)
}
