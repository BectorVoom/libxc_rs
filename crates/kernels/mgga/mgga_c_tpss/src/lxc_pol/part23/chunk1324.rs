//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1324/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1324<F: Float>(t1675: F, t1860: F, t1861: F, t19219: F, t19220: F, t19223: F, t19226: F, t19342: F, t19352: F, t19380: F, t20780: F, t5483: F, t5975: F, t5976: F, t5979: F, t6073: F, t6090: F, t63492: F, t63498: F, t65217: F, t65396: F) -> (F,) {
    let t68049 = -t5483 * t20780 / 3.0 - t1675 * t19219 * t6090 / 6.0 - t1675 * t5975 * t19380 / 3.0 - t1675 * t1860 * t65396 / 6.0 - 10.0 * t63492 * t19342 - 10.0 * t63498 * t19342 - t65217 * t1861 / 6.0 - t19352 * t5976 / 3.0 - t19352 * t5979 / 3.0 - t6073 * t19220 / 6.0 - t6073 * t19223 / 3.0 - t6073 * t19226 / 6.0;
    (t68049,)
}
