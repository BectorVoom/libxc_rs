//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1393/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1393<F: Float>(t1848: F, t4562: F, t20648: F, t550: F, t1284: F, t6441: F, t1276: F, t13253: F, t13292: F, t1666: F, t1673: F, t1849: F, t1856: F, t19011: F, t19050: F, t20697: F, t3413: F, t4544: F, t5942: F, t5960: F, t63114: F, t63173: F, t6442: F) -> F {
    let t67868 = F::cast_from(2.0_f64) * t1848 * t4562;
    let t67874 = F::cast_from(2.0_f64) * t20648 * t550;
    let t67879 = F::cast_from(2.0_f64) * t6441 * t1284;
    let t67881 = F::cast_from(2.0_f64) * t1276 * t20697 + t13253 * t1856 + t13292 * t1849 + t1666 * t19050 + t1673 * t19011 + t3413 * t6442 + F::cast_from(2.0_f64) * t4544 * t5960 + F::cast_from(2.0_f64) * t4562 * t5942 + F::cast_from(2.0_f64) * t63114 + F::cast_from(2.0_f64) * t63173 + t67868 + t67874 + t67879;
    t67881
}
