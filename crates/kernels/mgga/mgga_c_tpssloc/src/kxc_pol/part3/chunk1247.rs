//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1247/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1247<F: Float>(t16131: F, t16435: F, t1378: F, t225: F, t5319: F, t1372: F, t5210: F, t12030: F, t12444: F, t1375: F, t1386: F, t16022: F, t16028: F, t16030: F, t1843: F, t3758: F, t3889: F, t3912: F, t5215: F, t5321: F, t5354: F, t568: F) -> F {
    let t16436 = t16131 + t16435;
    let t16437 = t1378 * t16436;
    let t16439 = t5319 * t225;
    let t16448 = t5210 * t1372;
    let t16451 = -t12030 * t1843 - F::new(2.0) * t12444 * t1843 - t1375 * t16437 - F::new(2.0) * t1386 * t16022 - F::new(2.0) * t1386 * t16030 - F::new(2.0) * t1386 * t16439 + t16028 * t568 + F::new(2.0) * t16448 * t568 - F::new(2.0) * t3758 * t5354 + F::new(2.0) * t3889 * t5321 - t3912 * t5215 - t3912 * t5321;
    t16451
}
