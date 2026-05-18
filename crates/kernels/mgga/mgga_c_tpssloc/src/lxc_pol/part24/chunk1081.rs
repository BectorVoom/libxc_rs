//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1081/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1081<F: Float>(t12437: F, t1378: F, t12237: F, t562: F, t12434: F, t539: F, t225: F, t3755: F, t12016: F, t12023: F, t12027: F, t12030: F, t12033: F, t12036: F, t1375: F, t1386: F, t3758: F, t3882: F, t3889: F, t3912: F, t568: F) -> (F, F, F, F, F) {
    let t12438 = t1378 * t12437;
    let t12440 = t12237 * t562;
    let t12442 = t539 * t12434;
    let t12444 = t3755 * t225;
    let t12451 = F::new(3.0) * t12016 * t568 - F::new(6.0) * t12023 * t1375 + F::new(6.0) * t12027 * t1375 - F::new(3.0) * t12030 * t1386 - F::new(3.0) * t12033 * t1386 + F::new(3.0) * t12036 * t568 - t12438 * t1375 + t12440 * t568 + t12442 * t568 - F::new(6.0) * t12444 * t1386 + F::new(6.0) * t3758 * t3889 - F::new(3.0) * t3758 * t3912 + F::new(6.0) * t3882 * t3889 - F::new(3.0) * t3882 * t3912;
    (t12438, t12440, t12442, t12444, t12451)
}
