//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2273/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2273<F: Float>(t2379: F, t828: F, t41115: F, t4191: F, t41107: F, t4166: F, t9670: F, t831: F, t13210: F, t13228: F, t13254: F, t13333: F, t13350: F, t41130: F, t41132: F, t41134: F, t41139: F, t41237: F, t41341: F, t4167: F, t4172: F, t4178: F, t9618: F, t9642: F, t9960: F) -> F {
    let t47072 = t2379 * t828;
    let t47079 = t41115 * t4191;
    let t47080 = F::new(119.0) / F::new(1152.0) * t47079;
    let t47081 = t41107 * t4191;
    let t47092 = t4166 * t9670;
    let t47093 = t47092 * t831;
    let t47094 = F::new(119.0) / F::new(4608.0) * t47093;
    let t47097 = F::new(5.0) / F::new(128.0) * t4178 * t13350 * t13228 * t47072 + t9642 * t13210 / F::new(256.0) + t47080 - F::new(7.0) / F::new(192.0) * t47081 + F::new(3.0) / F::new(512.0) * t13254 * t13333 - F::new(595.0) / F::new(3456.0) * t41130 - F::new(7.0) / F::new(4608.0) * t41132 + F::new(119.0) / F::new(4608.0) * t41134 + t41139 + F::new(7.0) / F::new(4608.0) * t41237 - F::new(119.0) / F::new(2304.0) * t41341 + F::new(5.0) / F::new(256.0) * t4172 * t9618 - t47094 - t4167 * t9960 / F::new(3072.0);
    t47097
}
