//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1236/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1236<F: Float>(t1364: F, t1398: F, t14076: F, t1692: F, t1812: F, t18807: F, t18812: F, t198: F, t19809: F, t19818: F, t20509: F, t20514: F, t207: F, t2439: F, t3552: F, t3610: F, t3683: F, t3724: F, t5849: F, t5853: F, t6354: F, t750: F, t821: F, t823: F) -> F {
    let t20576 = t198 * t20509 * t207 * t823 + F::new(3.0) * t1364 * t2439 * t5849 - t1398 * t1692 * t18807 - F::new(3.0) * t14076 * t2439 * t5853 + F::new(2.0) * t1692 * t18812 * t19818 - t1692 * t20514 * t821 - t1692 * t3724 * t5853 + F::new(3.0) * t1812 * t2439 * t3610 + F::new(6.0) * t1812 * t3552 * t3683 - F::new(3.0) * t19809 * t2439 * t5853 + F::new(3.0) * t2439 * t6354 * t750;
    t20576
}
