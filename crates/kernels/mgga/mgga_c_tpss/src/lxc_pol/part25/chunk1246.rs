//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1246/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1246<F: Float>(t1692: F, t1812: F, t18728: F, t18812: F, t19681: F, t19829: F, t19836: F, t20417: F, t20514: F, t20526: F, t21255: F, t21270: F, t21659: F, t2439: F, t3552: F, t580: F, t5849: F, t5853: F, t6354: F, t69804: F, t69807: F, t69848: F, t69871: F, t69891: F, t70237: F, t70255: F, t70258: F, t70286: F) -> (F,) {
    let t72242 = 3.0 * t2439 * t1812 * t70286 + 3.0 * t3552 * t5849 * t21255 + t1692 * t21659 * t580 / 2.0 + 3.0 * t18728 * t69848 + t1692 * t18812 * t70258 + 3.0 * t2439 * t6354 * t19681 + 3.0 / 2.0 * t2439 * t5849 * t21270 + 6.0 * t20417 * t69807 + 6.0 * t18728 * t69804 + 2.0 * t20526 * t70237 - t1692 * t5853 * t69871 / 2.0 - t1692 * t20514 * t19836 + 3.0 / 2.0 * t2439 * t1812 * t69891 - t1692 * t5853 * t70255 + 3.0 * t2439 * t6354 * t19829;
    (t72242,)
}
