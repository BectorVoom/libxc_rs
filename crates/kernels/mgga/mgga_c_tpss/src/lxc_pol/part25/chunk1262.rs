//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1262/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1262<F: Float>(t21854: F, t509: F, t1270: F, t20226: F, t6245: F, t18686: F, t21017: F, t13627: F, t1845: F, t13955: F, t118: F, t1322: F, t13565: F, t1600: F, t1760: F, t1800: F, t1830: F, t1834: F, t1846: F, t21180: F, t21253: F, t21576: F, t21750: F, t21786: F, t21790: F, t4631: F, t4641: F, t4675: F, t485: F, t5463: F, t5801: F, t6243: F, t626: F, t6309: F, t6399: F, t6437: F) -> (F, F, F, F, F, F, F) {
    let t21855 = t509 * t21854;
    let t21856 = t21855 * t1270;
    let t21858 = t20226 * t6245;
    let t21863 = t18686 * t21017;
    let t21868 = t1845 * t13627;
    let t21871 = t1845 * t13955;
    let t21877 = -t118 * t21750 - F::new(2.0) * t1322 * t6399 - F::new(2.0) * t13565 * t1800 - F::new(2.0) * t1600 * t6309 - F::new(2.0) * t1760 * t21790 + t1760 * t21856 + F::new(6.0) * t1760 * t21858 + F::new(6.0) * t1760 * t21863 + F::new(2.0) * t1760 * t21868 - t1760 * t21871 - F::new(4.0) * t1800 * t21180 - t1830 * t4631 + t1834 * t5463 + t1846 * t21253 - F::new(4.0) * t21576 * t626 - t21786 * t485 - F::new(4.0) * t4641 * t5801 - F::new(2.0) * t4675 * t5801 + F::new(2.0) * t6243 * t6437;
    (t21855, t21856, t21858, t21863, t21868, t21871, t21877)
}
