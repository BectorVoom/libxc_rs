//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1354/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1354<F: Float>(t10514: F, t10552: F, t10662: F, t10667: F, t10897: F, t1692: F, t1812: F, t18728: F, t18812: F, t198: F, t19818: F, t20514: F, t207: F, t2133: F, t2428: F, t2433: F, t2439: F, t3552: F, t35525: F, t3683: F, t5849: F, t5853: F, t62807: F, t62829: F, t6354: F, t63844: F, t64248: F, t64296: F, t64770: F, t66299: F, t66603: F, t823: F) -> F {
    let t66750 = t198 * t207 * t66603 * t823 - F::new(3.0) * t2439 * t5853 * t35525 + F::new(12.0) * t3552 * t5849 * t3683 + F::new(2.0) * t1692 * t18812 * t63844 + F::new(3.0) * t2439 * t1812 * t10552 - F::new(6.0) * t2439 * t20514 * t10514 + F::new(2.0) * t1692 * t66299 * t2433 + F::new(12.0) * t3552 * t1812 * t10662 + F::new(6.0) * t3552 * t1812 * t10667 - F::new(6.0) * t1692 * t62807 * t64248 + F::new(3.0) * t2439 * t6354 * t2133 - F::new(3.0) * t2439 * t5853 * t64296 - t1692 * t5853 * t10897 + F::new(12.0) * t18728 * t64770 + F::new(4.0) * t1692 * t62829 * t19818 - t1692 * t20514 * t2428;
    t66750
}
