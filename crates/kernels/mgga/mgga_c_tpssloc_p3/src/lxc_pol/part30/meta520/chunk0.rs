//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1853/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1853<F: Float>(t5: F, t26054: F, t26095: F, t112: F, t1868: F, t671: F, t12725: F, t1873: F, t19456: F, t4028: F, t6534: F, t1458: F, t649: F) -> (F, F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t26097 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t26054 + t26095);
    let t26098 = t26097 * t112;
    let t26103 = t1868 * t671;
    let t26109 = F::cast_from(2.0_f64) * t12725 * t1873;
    let t26111 = F::cast_from(2.0_f64) * t19456 * t1873;
    let t26113 = F::cast_from(2.0_f64) * t4028 * t6534;
    let t26114 = t649 * t1458;
    (t26097, t26098, t26103, t26109, t26111, t26113, t26114)
}
