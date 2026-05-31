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
    let t47080 = F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t47079;
    let t47081 = t41107 * t4191;
    let t47092 = t4166 * t9670;
    let t47093 = t47092 * t831;
    let t47094 = F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t47093;
    let t47097 = F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t4178 * t13350 * t13228 * t47072 + t9642 * t13210 / F::cast_from(256.0_f64) + t47080 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t47081 + F::cast_from(3.0_f64) / F::cast_from(512.0_f64) * t13254 * t13333 - F::cast_from(595.0_f64) / F::cast_from(3456.0_f64) * t41130 - F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t41132 + F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t41134 + t41139 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t41237 - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t41341 + F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t4172 * t9618 - t47094 - t4167 * t9960 / F::cast_from(3072.0_f64);
    t47097
}
