//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2290/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2290<F: Float>(t1751: F, t24594: F, t24574: F, t27403: F, t1238: F, t1251: F, t14706: F, t15425: F, t15786: F, t1716: F, t2144: F, t2154: F, t2155: F, t24596: F, t24638: F, t24880: F, t24893: F, t27741: F, t3598: F, t4930: F, t498: F, t5060: F, t5089: F, t51925: F, t7283: F, t7285: F, t7286: F, t85688: F, t86451: F, t86456: F) -> F {
    let t94754 = t24594 * t1751;
    let t94759 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27403;
    let t94770 = F::new(2.0) * t1238 * t3598 * t2154 * t15786 + t15425 * t2144 * t498 - F::new(2.0) * t24893 * t5089 + F::new(4.0) * t1238 * t3598 * t27741 * t1251 - F::new(2.0) * t51925 * t2155 + F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t4930 * t24638 + F::cast_from(0.36554090374405031923e-2_f64) * t7283 * t94754 * t24596 - t94759 + t86451 - F::cast_from(0.91385225936012579807e-3_f64) * t86456 + F::new(4.0) * t24880 * t5060 - F::cast_from(0.16449340668482264365e-1_f64) * t7283 * t1716 * t85688 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t7285 * t7286 * t14706;
    t94770
}
