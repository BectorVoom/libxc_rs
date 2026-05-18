//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1358/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1358<F: Float>(t1692: F, t1812: F, t18250: F, t18265: F, t18728: F, t18803: F, t18807: F, t20021: F, t20041: F, t20065: F, t20417: F, t20526: F, t2439: F, t5849: F, t5853: F, t6207: F, t62610: F, t6354: F, t64880: F, t64928: F, t64950: F, t64954: F, t64958: F, t64969: F, t64979: F, t64992: F, t65002: F, t66299: F) -> F {
    let t66870 = -t1692 * t18807 * t20065 + F::new(3.0) / F::new(2.0) * t2439 * t1812 * t64950 - F::new(3.0) * t20526 * t64880 + F::new(3.0) * t2439 * t5849 * t20021 - F::new(3.0) * t18728 * t65002 + F::new(3.0) / F::new(2.0) * t2439 * t1812 * t64992 + F::new(3.0) / F::new(2.0) * t2439 * t18803 * t6207 - F::new(3.0) / F::new(2.0) * t18728 * t64969 + F::new(2.0) * t20526 * t64958 - F::new(6.0) * t20417 * t64928 + F::new(3.0) * t2439 * t6354 * t18250 - F::new(3.0) * t62610 * t20041 - F::new(3.0) / F::new(2.0) * t18728 * t64979 - t1692 * t5853 * t64954 + t1692 * t66299 * t18265;
    t66870
}
