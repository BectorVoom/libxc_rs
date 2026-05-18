//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1162/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1162<F: Float>(t13551: F, t16039: F, t3: F, t4637: F, t645: F, t3537: F, t4555: F, t116: F, t4674: F, t117: F, t13546: F, t1279: F, t1281: F, t1668: F, t1670: F, t4549: F, t4556: F, t4559: F, t547: F, t5470: F, t5474: F, t5477: F, t548: F) -> (F, F, F, F, F, F, F) {
    let t16040 = t13551 + t16039;
    let t16041 = t3 * t16040;
    let t16052 = param_d * t16040;
    let t16064 = t645 * t4637;
    let t16067 = t4555 * t3537;
    let t16072 = t116 * t4674;
    let t16073 = t16072 * t645;
    let t16076 = t117 * t13546;
    let t16079 = F::new(6.0) * t1279 * t5474 + F::new(3.0) * t1279 * t5477 + F::new(3.0) * t1281 * t5470 + t16052 * t548 + F::new(6.0) * t16064 * t547 + F::new(12.0) * t16067 * t547 + F::new(6.0) * t16073 * t547 + F::new(3.0) * t16076 * t547 + F::new(12.0) * t1668 * t4556 + F::new(6.0) * t1668 * t4559 + F::new(6.0) * t1670 * t4549;
    (t16041, t16052, t16064, t16067, t16073, t16076, t16079)
}
