//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 459/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk459<F: Float>(t2261: F, t42: F, t2244: F, t2250: F, t43: F, t54: F, t55: F, t240: F, t59: F, t39: F, t44: F, t51: F, t615: F, t618: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t2262 = sigma0 * t2261;
    let t2267 = F::new(1.0) / t42;
    let t2268 = t2267 * t2244;
    let t2271 = t43 * t2250;
    let t2274 = F::new(1.0) / t54;
    let t2275 = t2274 * t2244;
    let t2278 = t55 * t2250;
    let t2281 = t59 * t240;
    let t2282 = F::new(88.0) / F::new(9.0) * t2281;
    let t2283 = F::new(88.0) / F::new(9.0) * t2262 * t44 - F::new(40.0) / F::new(9.0) * t615 * t618 + F::new(5.0) / F::new(18.0) * t39 * t2268 + F::new(5.0) / F::new(6.0) * t39 * t2271 + F::new(5.0) / F::new(18.0) * t51 * t2275 - F::new(5.0) / F::new(6.0) * t51 * t2278 - t2282;
    (t2262, t2267, t2268, t2271, t2274, t2281, t2283)
}
