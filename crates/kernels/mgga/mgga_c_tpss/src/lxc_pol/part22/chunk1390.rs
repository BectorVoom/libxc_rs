//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1390/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1390<F: Float>(t67223: F, t67274: F, t67589: F, t67633: F, t67674: F, t67715: F, t67751: F, t67792: F, t1279: F, t13279: F, t13283: F, t13286: F, t13289: F, t1338: F, t1668: F, t1670: F, t1851: F, t19023: F, t19037: F, t19040: F, t20678: F, t20679: F, t20694: F, t3403: F, t3407: F, t3537: F, t4559: F, t547: F, t548: F, t5947: F, t63152: F, t6446: F, t6455: F, t66195: F, t66199: F) -> (F, F) {
    let t67795 = t67223 + t67274 + t67589 + t67633 + t67674 + t67715 + t67751 + t67792;
    let t67800 = F::new(6.0) * t1338 * t547 * t63152 + F::new(6.0) * t1338 * t547 * t66195 + F::new(12.0) * t1338 * t547 * t66199 + F::new(12.0) * t19040 * t3537 * t547 + F::new(12.0) * t20678 * t3537 * t547 + t548 * t67795 * param_d + F::new(12.0) * t1279 * t20679 + F::new(6.0) * t1279 * t20694 + F::new(6.0) * t13279 * t1851 + F::new(12.0) * t13283 * t1851 + F::new(6.0) * t13286 * t1851 + F::new(3.0) * t13289 * t1851 + F::new(6.0) * t1668 * t19037 + F::new(3.0) * t1670 * t19023 + F::new(3.0) * t3403 * t6455 + F::new(6.0) * t3407 * t6446 + F::new(6.0) * t4559 * t5947;
    (t67795, t67800)
}
