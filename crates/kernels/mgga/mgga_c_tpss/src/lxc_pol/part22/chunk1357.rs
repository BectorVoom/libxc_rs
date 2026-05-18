//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1357/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1357<F: Float>(t1692: F, t1812: F, t18239: F, t18728: F, t18807: F, t18812: F, t20025: F, t20054: F, t20417: F, t20526: F, t2439: F, t3552: F, t35530: F, t5849: F, t5853: F, t6214: F, t62820: F, t6354: F, t6380: F, t64870: F, t64876: F, t64905: F, t64914: F, t64917: F, t64941: F, t64946: F, t64976: F, t64986: F, t64997: F) -> F {
    let t66833 = F::new(3.0) * t2439 * t5849 * t20025 + t1692 * t18812 * t64876 + F::new(3.0) * t3552 * t6354 * t18239 - t1692 * t62820 * t6214 / F::new(2.0) + F::new(2.0) * t20526 * t64941 - F::new(3.0) * t18728 * t64976 + F::new(6.0) * t20417 * t64914 + F::new(6.0) * t20417 * t64997 + F::new(3.0) * t20417 * t64870 + F::new(3.0) / F::new(2.0) * t2439 * t1812 * t64905 - F::new(3.0) * t18728 * t64986 - t1692 * t5853 * t64946 / F::new(2.0) + F::new(3.0) * t3552 * t1812 * t64917 - t1692 * t18807 * t20054 + F::new(3.0) * t35530 * t6380;
    t66833
}
