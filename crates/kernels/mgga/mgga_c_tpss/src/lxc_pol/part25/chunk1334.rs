//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1334/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1334<F: Float>(t1163: F, t1273: F, t1322: F, t13627: F, t13856: F, t13955: F, t1760: F, t1846: F, t18547: F, t18690: F, t19579: F, t19620: F, t20289: F, t20379: F, t2056: F, t20640: F, t21253: F, t21897: F, t21907: F, t21944: F, t3491: F, t3499: F, t3537: F, t3538: F, t4674: F, t51622: F, t5314: F, t5815: F, t5895: F, t5910: F, t5936: F, t5937: F, t6103: F, t626: F, t6399: F, t67246: F, t68827: F, t68975: F, t7383: F) -> F {
    let t71303 = -F::new(2.0) * t626 * t1163 * t21907 - t1760 * t5936 * t13955 + t68975 * t1846 + t21253 * t5937 - F::new(3.0) * t18547 * t18690 * t51622 + F::new(6.0) * t19620 * t7383 * t13856 + F::new(2.0) * t1760 * t5936 * t13627 - F::new(2.0) * t3491 * t6399 - F::new(2.0) * t1322 * t20640 - F::new(4.0) * t20289 * t3538 - F::new(2.0) * t626 * t5314 * t5815 - F::new(2.0) * t2056 * t21897 - F::new(2.0) * t3499 * t21897 - F::new(2.0) * t626 * t5895 * t4674 - F::new(4.0) * t6103 * t20379 - F::new(4.0) * t626 * t6399 * t3537 + F::new(3.0) * t21253 * t5910 + t21944 * t1273 - F::new(6.0) * t19579 * t67246 * t68827;
    t71303
}
