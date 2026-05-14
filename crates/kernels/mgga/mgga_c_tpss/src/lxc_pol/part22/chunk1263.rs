//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1263/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1263<F: Float>(t1665: F, t5960: F, t1275: F, t6458: F, t1673: F, t5941: F, t20697: F, t546: F, t1856: F, t4543: F, t1278: F, t1284: F, t20649: F, t3: F, t3399: F, t550: F, t62171: F, t63116: F, t63167: F, t63169: F, t67795: F, t67800: F, t67843: F) -> (F,) {
    let t67849 = 2.0 * t1665 * t5960;
    let t67851 = 2.0 * t1275 * t6458;
    let t67853 = 2.0 * t5941 * t1673;
    let t67858 = 2.0 * t546 * t20697;
    let t67860 = 2.0 * t4543 * t1856;
    let t67861 = t1278 * (t67800 + t67843) + t62171 + t63169 + 2.0 * t20649 * t1284 + t67849 + t67851 + t63116 + t67853 + t3 * t67795 * t550 + t63167 + t3399 * t6458 + t67858 + t67860;
    (t67861,)
}
