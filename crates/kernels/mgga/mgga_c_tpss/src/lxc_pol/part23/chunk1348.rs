//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1348/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1348<F: Float>(t12664: F, t13131: F, t13225: F, t1339: F, t1600: F, t1865: F, t1899: F, t19240: F, t19261: F, t2062: F, t3502: F, t5986: F, t63626: F, t6540: F, t66033: F, t66035: F, t66038: F, t66042: F, t66046: F, t66048: F, t66050: F, t66054: F, t66056: F, t66059: F, t66060: F, t68163: F) -> (F,) {
    let t68742 = -t12664 * t1865 + t13131 * t1899 - 4.0 * t13225 * t5986 - 2.0 * t1339 * t63626 - 4.0 * t1339 * t68163 - t1600 * t19240 - 4.0 * t19261 * t3502 - 2.0 * t2062 * t6540 - t66033 - t66035 - t66038 + t66042 - t66046 - t66048 - t66050 - t66054 - t66056 - t66059 - t66060;
    (t68742,)
}
