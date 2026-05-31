//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1224/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1224<F: Float>(t10514: F, t1692: F, t1812: F, t18802: F, t18807: F, t18812: F, t198: F, t207: F, t2116: F, t2133: F, t2428: F, t2433: F, t2439: F, t3552: F, t5849: F, t5853: F, t750: F, t821: F, t823: F) -> F {
    let t18847 = t18802 * t198 * t207 * t823 - F::cast_from(6.0_f64) * t10514 * t2439 * t5853 - F::cast_from(2.0_f64) * t1692 * t18807 * t821 + F::cast_from(2.0_f64) * t1692 * t18812 * t2433 - t1692 * t2428 * t5853 + F::cast_from(6.0_f64) * t1812 * t2116 * t3552 + F::cast_from(3.0_f64) * t1812 * t2133 * t2439 + F::cast_from(6.0_f64) * t2439 * t5849 * t750;
    t18847
}
