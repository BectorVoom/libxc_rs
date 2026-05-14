//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1352/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1352<F: Float>(t1665: F, t6071: F, t1901: F, t4562: F, t1284: F, t6547: F, t1673: F, t6061: F, t1275: F, t6556: F, t13253: F, t1666: F, t1906: F, t19300: F, t3: F, t550: F, t6062: F, t63175: F, t63662: F, t63664: F, t63675: F, t68745: F) -> (F,) {
    let t68776 = 2.0 * t1665 * t6071;
    let t68780 = 2.0 * t1901 * t4562;
    let t68782 = 2.0 * t6547 * t1284;
    let t68786 = 2.0 * t6061 * t1673;
    let t68788 = 2.0 * t1275 * t6556;
    let t68792 = t3 * t550 * t68745 + t13253 * t1906 + t1666 * t19300 + 2.0 * t4562 * t6062 + 2.0 * t63175 + t63662 + t63664 + t63675 + t68776 + t68780 + t68782 + t68786 + t68788;
    (t68792,)
}
