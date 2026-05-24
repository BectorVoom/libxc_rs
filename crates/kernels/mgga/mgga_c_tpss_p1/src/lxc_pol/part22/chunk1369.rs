//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1369/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1369<F: Float>(t118: F, t12664: F, t1270: F, t12836: F, t12841: F, t1338: F, t13554: F, t1760: F, t1799: F, t18547: F, t18613: F, t18690: F, t18691: F, t18707: F, t18896: F, t19305: F, t19308: F, t19579: F, t19581: F, t19620: F, t20134: F, t20346: F, t2062: F, t26207: F, t3493: F, t42962: F, t4478: F, t509: F, t5816: F, t6103: F, t61801: F, t626: F, t6399: F, t65056: F, t65060: F, t65533: F, t66217: F, t66764: F, t66912: F, t66998: F, t67057: F, t67109: F, t67211: F, t7383: F) -> F {
    let t67223 = -F::new(6.0) * t18547 * t18690 * t42962 + F::new(4.0) * t19579 * t66217 * t19581 - F::new(6.0) * t65533 * t18691 + F::new(12.0) * t65056 * t20134 + F::new(12.0) * t19620 * t26207 * t4478 - t118 * (t66764 + t66912) - F::new(2.0) * t2062 * t6399 + F::new(12.0) * t19620 * t7383 * t12836 + F::new(6.0) * t19620 * t7383 * t12841 - F::new(3.0) * t18547 * t18690 * t65060 - F::new(6.0) * t61801 * t20346 - F::new(4.0) * t19305 * t5816 - F::new(4.0) * t19308 * t5816 - F::new(4.0) * t6103 * t18707 - F::new(4.0) * t13554 * t5816 - F::new(2.0) * t3493 * t18613 + t1760 * t509 * (t66998 + t67057 + t67109 + t67211) * t1270 - F::new(2.0) * t626 * t12664 * t1799 - F::new(2.0) * t626 * t18896 * t1338;
    t67223
}
